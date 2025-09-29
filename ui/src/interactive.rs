use chrono::Duration;
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
            "project" => self.cmd_project(&parts[1..]).await,
            "delete-project" => self.cmd_delete_project(&parts[1..]).await,

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
        println!("  project <name>              - Show details and sessions for a project");
        println!(
            "  delete-project <name>       - Delete all sessions for a project (irreversible!)"
        );
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
        println!("📂 Your Projects:");
        println!("==================");

        match self.core.get_projects().await {
            Ok(projects) => {
                if projects.is_empty() {
                    println!("No projects yet. Start working on one with 'work <project>'!");
                } else {
                    for (project, duration) in projects {
                        let total_minutes = duration.num_minutes();
                        let hours = total_minutes / 60;
                        let minutes = total_minutes % 60;
                        if hours > 0 {
                            println!("{}: {} hours {} minutes", project, hours, minutes);
                        } else {
                            println!("{}: {} minutes", project, minutes);
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Error loading projects: {}", e);
            }
        }

        println!();
        Ok(())
    }

    async fn cmd_today(&mut self) -> Result<(), PersistenceError> {
        println!("📅 Today's Work Summary:");
        println!("======================");

        match self.core.get_today_summary().await {
            Ok(summary) => {
                let mut today_projects: Vec<(String, chrono::Duration)> =
                    summary.into_iter().collect();
                today_projects.sort_by(|a, b| a.0.cmp(&b.0));

                if today_projects.is_empty() {
                    println!("No work sessions today yet. Get started with 'work <project>'!");
                } else {
                    for (project, duration) in today_projects {
                        let total_minutes = duration.num_minutes();
                        let hours = total_minutes / 60;
                        let minutes = total_minutes % 60;
                        if hours > 0 {
                            println!("{}: {} hours {} minutes", project, hours, minutes);
                        } else {
                            println!("{}: {} minutes", project, minutes);
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Error loading today's summary: {}", e);
            }
        }

        println!();
        Ok(())
    }

    async fn cmd_project(&mut self, args: &[&str]) -> Result<(), PersistenceError> {
        if args.is_empty() {
            println!("❌ Usage: project <project-name>");
            println!("   Example: project \"test_project\"");
            return Ok(());
        }

        let project = args[0].to_string();
        let sessions = self.core.get_sessions_for_project(&project).await?;

        if sessions.is_empty() {
            println!("❌ No sessions found for project '{}'.", project);
            return Ok(());
        }

        // Calculate total duration
        let total_duration: Duration = sessions.iter().map(|s| s.duration).sum();
        let total_minutes = total_duration.num_minutes();
        let total_hours = total_minutes / 60;
        let total_mins = total_minutes % 60;

        println!("📊 Project Details: {}", project);
        println!("=========================");
        if total_hours > 0 {
            println!("Total time: {} hours {} minutes", total_hours, total_mins);
        } else {
            println!("Total time: {} minutes", total_mins);
        }
        println!("\nSessions:");
        for session in sessions {
            let start_str = session.start_time.format("%Y-%m-%d %H:%M").to_string();
            let end_str = session
                .end_time
                .map(|end| end.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or("Ongoing".to_string());
            let dur_mins = session.duration.num_minutes();
            let dur_str = if dur_mins >= 60 {
                let h = dur_mins / 60;
                let m = dur_mins % 60;
                format!("{} hours {} minutes", h, m)
            } else {
                format!("{} minutes", dur_mins)
            };
            println!(
                "- {}: End: {} ({}) | Description: {}",
                start_str,
                end_str,
                dur_str,
                session.description.as_deref().unwrap_or("None")
            );
        }
        println!();
        Ok(())
    }

    async fn cmd_delete_project(&mut self, args: &[&str]) -> Result<(), PersistenceError> {
        if args.is_empty() {
            println!("❌ Usage: delete-project <project-name>");
            println!("   ⚠️  This deletes ALL sessions for the project!");
            return Ok(());
        }

        let project = args[0].to_string();
        println!("⚠️  About to delete all sessions for '{}'.", project);
        println!("   This is irreversible! Type 'y' to confirm:");

        // Simple y/n prompt (using readline)
        let confirm = self.editor.readline("Confirm (y/n): ").ok();
        if confirm.as_deref() != Some("y") && confirm.as_deref() != Some("Y") {
            println!("❌ Deletion cancelled.");
            return Ok(());
        }

        match self.core.delete_project_sessions(&project).await {
            Ok(deleted) => {
                if deleted > 0 {
                    println!("✅ Deleted {} session(s) for '{}'.", deleted, project);
                    println!("💾 Data updated.");
                } else {
                    println!("ℹ️  No sessions found for '{}' to delete.", project);
                }
            }
            Err(e) => println!("❌ Error deleting: {}", e),
        }
        println!();
        Ok(())
    }
}
