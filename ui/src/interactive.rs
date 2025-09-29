use chrono::Duration;
use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
};
use netupi_core::{NetupiCore, PersistenceError, TimerState, TimerType};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::collections::HashMap;
use std::io::{Write, stdout};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::time::{Duration as TokioDuration, interval};

pub struct InteractiveMode {
    core: NetupiCore,
    editor: DefaultEditor,
    commands: HashMap<String, String>, // command -> description
    timer_display_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
    timer_stop_sender: Arc<RwLock<Option<mpsc::UnboundedSender<()>>>>,
    timer_display_line: Arc<RwLock<u16>>, // Which line to display the timer on
}

impl InteractiveMode {
    pub async fn new() -> Result<Self, PersistenceError> {
        let core = NetupiCore::new().await?;
        let mut editor = DefaultEditor::new().map_err(|e| {
            PersistenceError::Config(format!("Failed to initialize readline: {}", e))
        })?;

        // Load command history
        #[cfg(feature = "with-file-history")]
        if let Err(_) = editor.load_history(".netupi_history") {
            // No previous history, that's ok
        }

        // Build command help map
        let mut commands = HashMap::new();
        commands.insert(
            "work".to_string(),
            "Start tracking work time on a project".to_string(),
        );
        commands.insert(
            "pomodoro".to_string(),
            "Start a 25-minute pomodoro session".to_string(),
        );
        commands.insert("break".to_string(), "Start a 5-minute break".to_string());
        commands.insert("stop".to_string(), "Stop the current timer".to_string());
        commands.insert("pause".to_string(), "Pause the current timer".to_string());
        commands.insert("resume".to_string(), "Resume the paused timer".to_string());
        commands.insert("projects".to_string(), "List all your projects".to_string());
        commands.insert("today".to_string(), "Show today's work summary".to_string());
        commands.insert("help".to_string(), "Show available commands".to_string());
        commands.insert("clear".to_string(), "Clear the screen".to_string());
        commands.insert("exit".to_string(), "Exit Netupi".to_string());

        let interactive = Self {
            core,
            editor,
            commands,
            timer_display_handle: Arc::new(RwLock::new(None)),
            timer_stop_sender: Arc::new(RwLock::new(None)),
            timer_display_line: Arc::new(RwLock::new(0)),
        };

        Ok(interactive)
    }

    pub async fn run(&mut self) -> Result<(), PersistenceError> {
        self.print_welcome().await;

        loop {
            let prompt = "netupi> ";
            let readline = self.editor.readline(&prompt);

            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    // Add to history
                    if let Err(_) = self.editor.add_history_entry(line) {
                        // History error, continue anyway
                    }

                    // Parse and handle command
                    if let Err(e) = self.handle_command(line).await {
                        println!("❌ Error: {}", e);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("👋 Use 'exit' to quit or Ctrl+D");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("\n👋 Goodbye!");
                    break;
                }
                Err(err) => {
                    println!("❌ Input error: {}", err);
                    break;
                }
            }
        }

        // Stop any running timer display
        self.stop_timer_display().await;

        // Save history before exiting
        #[cfg(feature = "with-file-history")]
        if let Err(_) = self.editor.save_history(".netupi_history") {
            // History save failed, but don't error out
        }

        Ok(())
    }

    async fn start_timer_display(&mut self) {
        // Stop any existing display first
        self.stop_timer_display().await;

        let core = self.core.clone();
        let (stop_sender, mut stop_receiver) = mpsc::unbounded_channel();
        let timer_line = self.timer_display_line.clone();

        // Reserve a line above the prompt for the timer display
        println!(); // Add empty line where timer will be displayed
        *timer_line.write().await = 1; // Track that we're using line 1 above prompt

        let handle = tokio::spawn(async move {
            let mut interval = interval(TokioDuration::from_secs(1));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let state = core.timer().get_current_state().await;

                        if state.state == TimerState::Idle {
                            // Timer stopped, clear the display line and exit
                            if let Err(_) = Self::clear_timer_line(*timer_line.read().await).await {
                                // Error clearing, but continue
                            }
                            break;
                        }

                        let minutes = state.elapsed.num_minutes();
                        let seconds = state.elapsed.num_seconds() % 60;
                        let hours = minutes / 60;
                        let remaining_minutes = minutes % 60;

                        let timer_type_icon = match state.timer_type {
                            TimerType::PomodoroWork => "🍅",
                            TimerType::PomodoroShortBreak => "☕",
                            TimerType::Stopwatch => "⚡",
                            _ => "⏰",
                        };

                        let display = if hours > 0 {
                            format!("   {} Running: {:02}:{:02}:{:02}", timer_type_icon, hours, remaining_minutes, seconds)
                        } else {
                            format!("   {} Running: {:02}:{:02}", timer_type_icon, remaining_minutes, seconds)
                        };

                        // Update the timer display line
                        if let Err(_) = Self::update_timer_line(&display, *timer_line.read().await).await {
                            // Error updating, but continue
                        }
                    }
                    _ = stop_receiver.recv() => {
                        // Clear the timer line and exit
                        if let Err(_) = Self::clear_timer_line(*timer_line.read().await).await {
                            // Error clearing, but continue
                        }
                        break;
                    }
                }
            }
        });

        *self.timer_display_handle.write().await = Some(handle);
        *self.timer_stop_sender.write().await = Some(stop_sender);
    }

    async fn stop_timer_display(&mut self) {
        // Send stop signal
        if let Some(sender) = self.timer_stop_sender.write().await.take() {
            let _ = sender.send(());
        }

        // Wait for task to finish
        if let Some(handle) = self.timer_display_handle.write().await.take() {
            let _ = handle.await;
        }

        // Reset the timer line tracker
        *self.timer_display_line.write().await = 0;
    }

    async fn update_timer_line(content: &str, lines_up: u16) -> Result<(), std::io::Error> {
        let mut stdout = stdout();

        // Save current cursor position, move up, print content, restore position
        execute!(
            stdout,
            MoveUp(lines_up),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::Yellow),
            Print(content),
            ResetColor,
            MoveToColumn(0),
            MoveUp(lines_up.saturating_sub(lines_up)), // Move back to original position
        )?;

        stdout.flush()?;
        Ok(())
    }

    async fn clear_timer_line(lines_up: u16) -> Result<(), std::io::Error> {
        if lines_up == 0 {
            return Ok(());
        }

        let mut stdout = stdout();
        execute!(
            stdout,
            MoveUp(lines_up),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            MoveToColumn(0),
        )?;
        stdout.flush()?;
        Ok(())
    }

    async fn stop_current_timer_if_running(&mut self) -> Result<bool, PersistenceError> {
        let state = self.core.timer().get_current_state().await;

        if state.state != TimerState::Idle {
            // Stop the timer display first
            self.stop_timer_display().await;

            let total_minutes = state.elapsed.num_minutes();
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;

            self.core.timer().stop_timer().await?;

            println!("{}", "⏹️  Previous timer stopped early!".yellow());
            if hours > 0 {
                println!(
                    "{}",
                    format!("⏱️  Time recorded: {} hours {} minutes", hours, minutes).blue()
                );
            } else {
                println!(
                    "{}",
                    format!("⏱️  Time recorded: {} minutes", minutes).blue()
                );
            }
            println!("{}", "💾 Session saved successfully.".green());
            println!(); // Add spacing

            return Ok(true);
        }

        Ok(false)
    }
}

impl InteractiveMode {
    async fn print_welcome(&self) {
        println!(
            "{}",
            "🍅 Netupi23 - Interactive Time Tracker".green().bold()
        );
        println!("{}", "=====================================".green());
        println!(
            "Welcome! Type {} for commands or {} to quit.",
            "help".cyan(),
            "exit".cyan()
        );
        println!();
    }

    async fn handle_command(&mut self, input: &str) -> Result<(), PersistenceError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match command.as_str() {
            "work" | "w" => self.cmd_work(args).await,
            "pomodoro" | "pomo" => self.cmd_pomodoro().await,
            "break" | "br" => self.cmd_break().await,
            "stop" | "s" => self.cmd_stop().await,
            "pause" => self.cmd_pause().await,
            "resume" => self.cmd_resume().await,

            "projects" | "proj" => self.cmd_projects().await,
            "today" => self.cmd_today().await,
            "help" | "h" | "?" => self.cmd_help(),
            "clear" | "cls" => self.cmd_clear(),
            "exit" | "quit" | "q" => {
                self.stop_timer_display().await;
                println!("👋 Goodbye!");
                std::process::exit(0);
            }
            _ => {
                println!(
                    "❌ Unknown command: '{}'. Type 'help' for available commands.",
                    command
                );
                Ok(())
            }
        }
    }

    async fn cmd_work(&mut self, args: &[&str]) -> Result<(), PersistenceError> {
        if args.is_empty() {
            println!("❌ Usage: work <project-name> [description]");
            println!("   Example: work \"Mobile App\" \"implementing login screen\"");
            return Ok(());
        }

        // Stop current timer if running
        let _had_running_timer = self.stop_current_timer_if_running().await?;

        let project = args[0].to_string();
        let description = if args.len() > 1 {
            Some(args[1..].join(" "))
        } else {
            None
        };

        println!(
            "{}",
            format!("🚀 Starting work session for project: {}", project).green()
        );
        if let Some(desc) = &description {
            println!("{}", format!("📝 Description: {}", desc).blue());
        }

        self.core.start_work_session(project, description).await?;

        println!(
            "{}",
            "⏰ Work timer started! Use 'stop' to finish.".yellow()
        );

        // Start real-time display
        self.start_timer_display().await;

        Ok(())
    }

    async fn cmd_pomodoro(&mut self) -> Result<(), PersistenceError> {
        // Stop current timer if running (handles the auto-stop feature)
        let _had_running_timer = self.stop_current_timer_if_running().await?;

        println!("{}", "🍅 Starting 25-minute Pomodoro session...".green());
        self.core
            .timer()
            .start_timer(TimerType::PomodoroWork)
            .await?;

        println!(
            "{}",
            "⏰ Pomodoro started! Use 'stop' to finish early.".yellow()
        );

        // Start real-time display
        self.start_timer_display().await;

        Ok(())
    }

    async fn cmd_break(&mut self) -> Result<(), PersistenceError> {
        // Stop current timer if running (handles the auto-stop feature)
        let _had_running_timer = self.stop_current_timer_if_running().await?;

        println!("{}", "☕ Starting 5-minute break...".green());
        self.core
            .timer()
            .start_timer(TimerType::PomodoroShortBreak)
            .await?;

        println!(
            "{}",
            "⏰ Break started! Use 'stop' to finish early.".yellow()
        );

        // Start real-time display
        self.start_timer_display().await;

        Ok(())
    }

    async fn cmd_stop(&mut self) -> Result<(), PersistenceError> {
        let state = self.core.timer().get_current_state().await;

        if state.state == TimerState::Idle {
            println!("{}", "⏸️  No timer is currently running.".yellow());
            return Ok(());
        }

        // Stop the timer display first
        self.stop_timer_display().await;

        let total_minutes = state.elapsed.num_minutes();
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        self.core.timer().stop_timer().await?;

        println!("{}", "✅ Timer stopped!".green());
        if hours > 0 {
            println!(
                "{}",
                format!("⏱️  Total time: {} hours {} minutes", hours, minutes).blue()
            );
        } else {
            println!("{}", format!("⏱️  Total time: {} minutes", minutes).blue());
        }
        println!("{}", "💾 Session saved successfully.".green());

        Ok(())
    }

    fn cmd_help(&self) -> Result<(), PersistenceError> {
        println!("{}", "📚 Available Commands:".cyan().bold());
        println!("{}", "─".repeat(50).cyan());

        // Use the commands HashMap we built!
        for (cmd, desc) in &self.commands {
            println!("  {:<15} {}", cmd.clone().yellow(), desc);
        }

        println!();
        println!(
            "💡 {}",
            "Tip: Use arrow keys to navigate command history!".dim()
        );
        println!(
            "💡 {}",
            "Tip: Use Ctrl+C to interrupt, Ctrl+D or 'exit' to quit".dim()
        );
        println!(
            "💡 {}",
            "Tip: Starting a new timer will automatically stop the current one!".dim()
        );

        Ok(())
    }

    fn cmd_clear(&self) -> Result<(), PersistenceError> {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        Ok(())
    }

    async fn cmd_pause(&mut self) -> Result<(), PersistenceError> {
        println!("{}", "⏸️  Pause functionality coming soon!".yellow());
        // TODO: Implement pause functionality
        Ok(())
    }

    async fn cmd_resume(&mut self) -> Result<(), PersistenceError> {
        println!("{}", "▶️  Resume functionality coming soon!".yellow());
        // TODO: Implement resume functionality
        Ok(())
    }

    async fn cmd_projects(&mut self) -> Result<(), PersistenceError> {
        println!("{}", "📂 Your Projects:".cyan().bold());
        println!("{}", "─".repeat(30).cyan());

        // Get all sessions and extract project names
        let sessions = self.core.get_sessions().await.unwrap_or_default();
        let mut projects: std::collections::HashSet<String> = std::collections::HashSet::new();

        for session in &sessions {
            if let Some(tags) = session.tags.first() {
                projects.insert(tags.clone());
            }
        }

        if projects.is_empty() {
            println!(
                "{}",
                "  No projects found. Start tracking time with 'work <project-name>'".dim()
            );
        } else {
            for project in projects.iter() {
                // Calculate total time for this project
                let total_duration: Duration = sessions
                    .iter()
                    .filter(|s| s.tags.first().map_or(false, |tag| tag == project))
                    .map(|s| s.duration)
                    .sum();

                let total_minutes = total_duration.num_minutes();
                let hours = total_minutes / 60;
                let minutes = total_minutes % 60;

                if hours > 0 {
                    println!(
                        "  📋 {} ({} hours {} minutes)",
                        project.clone().yellow(),
                        hours,
                        minutes
                    );
                } else {
                    println!("  📋 {} ({} minutes)", project.clone().yellow(), minutes);
                }
            }
        }

        println!();
        println!(
            "{}",
            "💡 Use 'work <project-name>' to start tracking time for a project".dim()
        );
        Ok(())
    }

    async fn cmd_today(&mut self) -> Result<(), PersistenceError> {
        println!(
            "{}",
            "📅 Today's Summary (Coming in next iteration)".yellow()
        );
        println!("This will show today's work time breakdown.");
        Ok(())
    }
}
