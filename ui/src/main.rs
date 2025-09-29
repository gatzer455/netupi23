use clap::{Parser, Subcommand};
use netupi_core::{NetupiCore, PersistenceError, TimerState, TimerType};
use tokio::time::{Duration as TokioDuration, sleep};

mod interactive;
use interactive::InteractiveMode;

#[derive(Parser)]
#[command(name = "netupi")]
#[command(about = "🌻 A minimalist Pomodoro timer with persistence")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive mode
    Interactive,
    /// Start a Pomodoro work session (25 minutes)
    Work,
    /// Start a short break (5 minutes)
    Break,
    /// Start a long break (15 minutes)
    LongBreak,
    /// Start a stopwatch (no time limit)
    Stopwatch,
    /// Start tracking work time on a specific project
    TimeTrack {
        /// Name of the project you're working on
        project: String,
        /// Optional description of what you're working on
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Stop the current timer and save the session
    Stop,
    /// Show work time statistics
    Log,
    /// List all projects you've worked on
    Projects,
    /// Show today's work summary
    Today,
    /// Show details for a specific project
    Project {
        /// Project name
        project: String,
    },
    /// Delete all sessions for a specific project
    DeleteProject {
        /// Project name
        project: String,
    },
    /// Generate work time reports
    Report {
        /// Filter by specific project
        #[arg(short, long)]
        project: Option<String>,
        /// Number of days to include in report
        #[arg(short, long, default_value = "7")]
        days: u32,
    },
    /// Show current timer status
    Status,
}

#[tokio::main]
async fn main() -> Result<(), PersistenceError> {
    let cli = Cli::parse();

    // If no command provided, start interactive mode
    match cli.command {
        None => {
            let mut interactive = InteractiveMode::new().await?;
            interactive.run().await?;
        }
        Some(Commands::Interactive) => {
            let mut interactive = InteractiveMode::new().await?;
            interactive.run().await?;
        }
        Some(command) => {
            // Handle single commands
            handle_single_command(command).await?;
        }
    }

    Ok(())
}

async fn handle_single_command(command: Commands) -> Result<(), PersistenceError> {
    println!("🌻 Netupi23 - Minimalist Timer");
    println!("==============================");

    let mut core = NetupiCore::new().await?;

    match command {
        Commands::Interactive => unreachable!(), // handled above
        Commands::Work => start_timer(&mut core, TimerType::PomodoroWork, "work session").await,
        Commands::Break => {
            start_timer(&mut core, TimerType::PomodoroShortBreak, "short break").await
        }
        Commands::LongBreak => {
            start_timer(&mut core, TimerType::PomodoroLongBreak, "long break").await
        }
        Commands::Stopwatch => start_timer(&mut core, TimerType::Stopwatch, "stopwatch").await,
        Commands::TimeTrack {
            project,
            description,
        } => start_project_timer(&mut core, project, description).await,
        Commands::Stop => stop_current_timer(&mut core).await,
        Commands::Log => show_work_log(&mut core).await,
        Commands::Projects => show_projects(&mut core).await,
        Commands::Today => show_today_summary(&mut core).await,
        Commands::Project { project } => show_project_details(&mut core, project).await,
        Commands::DeleteProject { project } => delete_project(&mut core, project).await,
        Commands::Report { project, days } => generate_report(&mut core, project, days).await,
        Commands::Status => show_status(&mut core).await,
    }
}

async fn start_timer(
    core: &mut NetupiCore,
    timer_type: TimerType,
    name: &str,
) -> Result<(), PersistenceError> {
    println!("Starting {}...", name);
    core.timer().start_timer(timer_type).await?;

    println!("Timer started! Press Ctrl+C to stop.");
    loop {
        let state = core.timer().get_current_state().await;
        let minutes = state.elapsed.num_minutes();
        let seconds = state.elapsed.num_seconds() % 60;

        print!("\r⏱️  {:02}:{:02}", minutes, seconds);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        sleep(TokioDuration::from_secs(1)).await;
    }
}

async fn start_project_timer(
    core: &mut NetupiCore,
    project: String,
    description: Option<String>,
) -> Result<(), PersistenceError> {
    println!("🚀 Starting work session for project: {}", project);
    if let Some(desc) = &description {
        println!("📝 Description: {}", desc);
    }
    let project_display = project.clone();

    core.start_work_session(project, description).await?;
    println!("⏰ Work timer started! Use 'netupi stop' to finish and save your session.");

    loop {
        let state = core.timer().get_current_state().await;
        let minutes = state.elapsed.num_minutes();
        let seconds = state.elapsed.num_seconds() % 60;
        let hours = minutes / 60;
        let remaining_minutes = minutes % 60;

        if hours > 0 {
            print!(
                "\r🏗️  {} | {:02}:{:02}:{:02}",
                project_display, hours, remaining_minutes, seconds
            );
        } else {
            print!(
                "\r🏗️  {} | {:02}:{:02}",
                project_display, remaining_minutes, seconds
            );
        }
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn stop_current_timer(core: &mut NetupiCore) -> Result<(), PersistenceError> {
    let state = core.timer().get_current_state().await;

    if state.state == TimerState::Idle {
        println!("⏸️  No timer is currently running.");
        return Ok(());
    }

    let total_minutes = state.elapsed.num_minutes();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    core.timer().stop_timer().await?;

    println!("✅ Timer stopped!");
    if hours > 0 {
        println!("⏱️  Total time: {} hours {} minutes", hours, minutes);
    } else {
        println!("⏱️  Total time: {} minutes", minutes);
    }
    println!("💾 Session saved successfully.");

    Ok(())
}

async fn show_status(core: &mut NetupiCore) -> Result<(), PersistenceError> {
    let state = core.timer().get_current_state().await;
    println!("Timer Type: {:?}", state.timer_type);
    println!("State: {:?}", state.state);
    println!(
        "Elapsed: {:02}:{:02}",
        state.elapsed.num_minutes(),
        state.elapsed.num_seconds() % 60
    );
    Ok(())
}

async fn show_work_log(_core: &mut NetupiCore) -> Result<(), PersistenceError> {
    println!("📊 Work Log (Coming in next iteration)");
    println!("This will show your work time statistics.");
    Ok(())
}

async fn show_projects(core: &mut NetupiCore) -> Result<(), PersistenceError> {
    println!("📂 Your Projects:");
    println!("==================");

    match core.get_projects().await {
        Ok(projects) => {
            if projects.is_empty() {
                println!(
                    "No projects yet. Start working on one with 'netupi time-track <project>'!"
                );
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

async fn show_today_summary(core: &mut NetupiCore) -> Result<(), PersistenceError> {
    println!("📅 Today's Work Summary:");
    println!("======================");

    match core.get_today_summary().await {
        Ok(summary) => {
            let mut today_projects: Vec<(String, chrono::Duration)> = summary.into_iter().collect();
            today_projects.sort_by(|a, b| a.0.cmp(&b.0));

            if today_projects.is_empty() {
                println!("No work sessions today yet. Get started with 'netupi work <project>'!");
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

async fn show_project_details(core: &mut NetupiCore, project: String) -> Result<(), PersistenceError> {
    let sessions = core.get_sessions_for_project(&project).await?;

    if sessions.is_empty() {
        println!("❌ No sessions found for project '{}'.", project);
        return Ok(());
    }

    // Calculate total duration
    let total_duration: chrono::Duration = sessions.iter().map(|s| s.duration).sum();
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
        let end_str = session.end_time
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


async fn delete_project(core: &mut NetupiCore, project: String) -> Result<(), PersistenceError> {
    println!("⚠️  Deleting all sessions for '{}' (irreversible).", project);
    match core.delete_project_sessions(&project).await {
        Ok(deleted) => {
            if deleted > 0 {
                println!("✅ Deleted {} session(s).", deleted);
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


async fn generate_report(
    _core: &mut NetupiCore,
    project: Option<String>,
    days: u32,
) -> Result<(), PersistenceError> {
    match project {
        Some(proj) => println!("📈 Report for project '{}' (last {} days)", proj, days),
        None => println!("📈 Work Report (last {} days)", days),
    }
    println!("This will generate detailed work reports.");
    Ok(())
}
