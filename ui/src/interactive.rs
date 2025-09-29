use crossterm::style::Stylize;
use netupi_core::{NetupiCore, PersistenceError, TimerState, TimerType};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::sync::Arc;

pub struct InteractiveMode {
    core: Arc<NetupiCore>, // Fix: Changed from NetupiCore to Arc<NetupiCore>
    editor: DefaultEditor,
}

impl InteractiveMode {
    pub async fn new() -> Result<Self, PersistenceError> {
        let core = Arc::new(NetupiCore::new().await?);
        let mut editor = DefaultEditor::new().map_err(|e| {
            PersistenceError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create editor: {}", e),
            ))
        })?;

        // Load history
        #[cfg(feature = "with-file-history")]
        if let Err(_) = editor.load_history(".netupi_history") {
            // History doesn't exist yet, that's ok
        }

        Ok(Self { core, editor })
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

        // Save history before exiting
        #[cfg(feature = "with-file-history")]
        if let Err(_) = self.editor.save_history(".netupi_history") {
            // History save failed, but don't error out
        }

        Ok(())
    }

    async fn stop_current_timer_if_running(&mut self) -> Result<bool, PersistenceError> {
        let state = self.core.timer().get_current_state().await;

        if state.state == TimerState::Idle {
            return Ok(false);
        }

        let total_minutes = state.elapsed.num_minutes();
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        self.core.timer().stop_timer().await?;

        println!("✅ Timer stopped!");
        if hours > 0 {
            println!("⏱️  Total time: {} hours {} minutes", hours, minutes);
        } else {
            println!("⏱️  Total time: {} minutes", minutes);
        }
        println!("💾 Session saved successfully.");

        Ok(true)
    }
}

impl InteractiveMode {
    async fn print_welcome(&self) {
        println!();
        println!(
            "{}",
            "🌻 Netupi23 - Interactive Time Tracker".green().bold()
        );
        println!("======================================");
        println!("Type 'help' for available commands or 'exit' to quit.");
        println!();
    }

    async fn handle_command(&mut self, line: &str) -> Result<(), PersistenceError> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        match parts[0].to_lowercase().as_str() {
            "work" => self.cmd_work(&parts[1..]).await,
            "pomodoro" | "pomo" => self.cmd_pomodoro().await,
            "break" => self.cmd_break().await,
            "stop" => self.cmd_stop().await,
            "status" | "s" => self.cmd_status().await,
            "help" | "h" => {
                self.cmd_help();
                Ok(())
            }
            "clear" | "cls" => {
                self.cmd_clear();
                Ok(())
            }
            "pause" => self.cmd_pause().await,
            "resume" => self.cmd_resume().await,
            "projects" => self.cmd_projects().await,
            "today" => self.cmd_today().await,
            "exit" | "quit" | "q" => std::process::exit(0),
            _ => {
                println!(
                    "❓ Unknown command: '{}'. Type 'help' for available commands.",
                    parts[0]
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
            "⏰ Work timer started! Use 'stop' to finish or 'status' to check progress.".yellow()
        );

        Ok(())
    }

    async fn cmd_pomodoro(&mut self) -> Result<(), PersistenceError> {
        // Stop current timer if running
        let _had_running_timer = self.stop_current_timer_if_running().await?;

        println!("{}", "🍅 Starting 25-minute Pomodoro session...".green());
        self.core
            .timer()
            .start_timer(TimerType::PomodoroWork)
            .await?;

        println!(
            "{}",
            "⏰ Pomodoro started! Use 'stop' to finish early or 'status' to check progress."
                .yellow()
        );

        Ok(())
    }

    async fn cmd_break(&mut self) -> Result<(), PersistenceError> {
        let _had_running_timer = self.stop_current_timer_if_running().await?;

        println!("{}", "☕ Starting short break...".green());
        self.core
            .timer()
            .start_timer(TimerType::PomodoroShortBreak)
            .await?;

        println!(
            "{}",
            "⏰ Break started! Use 'stop' to finish early.".yellow()
        );

        Ok(())
    }

    async fn cmd_stop(&mut self) -> Result<(), PersistenceError> {
        self.stop_current_timer_if_running().await?;
        Ok(())
    }

    async fn cmd_status(&mut self) -> Result<(), PersistenceError> {
        let state = self.core.timer().get_current_state().await;

        if state.state == TimerState::Idle {
            println!("⏸️  No timer is currently running.");
            return Ok(());
        }

        let minutes = state.elapsed.num_minutes();
        let seconds = state.elapsed.num_seconds() % 60;
        let hours = minutes / 60;
        let remaining_minutes = minutes % 60;

        let timer_type_icon = match state.timer_type {
            TimerType::PomodoroWork => "🍅",
            TimerType::PomodoroShortBreak => "☕",
            TimerType::PomodoroLongBreak => "🛌",
            TimerType::Stopwatch => "⚡",
            _ => "⏰",
        };

        let timer_type_name = match state.timer_type {
            TimerType::PomodoroWork => "Pomodoro Work Session",
            TimerType::PomodoroShortBreak => "Short Break",
            TimerType::PomodoroLongBreak => "Long Break",
            TimerType::Stopwatch => "Work Session",
            _ => "Timer",
        };

        println!(
            "{} {} - State: {:?}",
            timer_type_icon, timer_type_name, state.state
        );

        if hours > 0 {
            println!(
                "⏱️  Elapsed time: {:02}:{:02}:{:02}",
                hours, remaining_minutes, seconds
            );
        } else {
            println!("⏱️  Elapsed time: {:02}:{:02}", remaining_minutes, seconds);
        }

        Ok(())
    }

    fn cmd_help(&self) {
        println!("🌻 Netupi23 Commands:");
        println!("  work <project> [description] - Start work timer for a project");
        println!("  pomodoro (or pomo)          - Start 25-minute Pomodoro session");
        println!("  break                       - Start short break timer");
        println!("  stop                        - Stop current timer and save session");
        println!("  status (or s)               - Show current timer status");
        println!("  pause                       - Pause current timer");
        println!("  resume                      - Resume paused timer");
        println!("  projects                    - List all projects");
        println!("  today                       - Show today's work summary");
        println!("  clear (or cls)              - Clear screen");
        println!("  help (or h)                 - Show this help");
        println!("  exit/quit (or q)            - Exit interactive mode");
        println!();
        println!("💡 Tip: Use 'status' or 's' to check your timer progress anytime!");
    }

    fn cmd_clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    async fn cmd_pause(&mut self) -> Result<(), PersistenceError> {
        println!("⏸️ Pause functionality coming soon!");
        Ok(())
    }

    async fn cmd_resume(&mut self) -> Result<(), PersistenceError> {
        println!("▶️ Resume functionality coming soon!");
        Ok(())
    }

    async fn cmd_projects(&mut self) -> Result<(), PersistenceError> {
        println!("📂 Projects functionality coming soon!");
        Ok(())
    }

    async fn cmd_today(&mut self) -> Result<(), PersistenceError> {
        println!("📅 Today's summary functionality coming soon!");
        Ok(())
    }
}
