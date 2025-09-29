use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{RwLock, mpsc};

// =============================================================================
// TYPES & ENUMS (Future module: types.rs)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimerType {
    Stopwatch,
    PomodoroWork,
    PomodoroShortBreak,
    PomodoroLongBreak,
    CustomTimer,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Completed,
}

#[derive(Debug, Clone)]
pub enum TimerEvent {
    Started,
    Paused,
    Resumed,
    Stopped,
    Completed,
    Tick(Duration),
}

// =============================================================================
// DATA MODELS (Future module: models.rs)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSession {
    pub id: uuid::Uuid,
    pub session_type: TimerType,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Duration,
    pub paused_duration: Duration,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerConfig {
    pub pomodoro_work_duration: Duration,
    pub pomodoro_short_break_duration: Duration,
    pub pomodoro_long_break_duration: Duration,
    pub long_break_interval: u32,
    pub auto_start_breaks: bool,
    pub auto_start_work: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerData {
    pub timer_type: TimerType,
    pub state: TimerState,
    pub target_duration: Option<Duration>,
    pub elapsed: Duration,
    pub paused_time: Duration,
    pub start_time: Option<DateTime<Utc>>,
    pub pause_start: Option<DateTime<Utc>>,
}

// =============================================================================
// ERROR TYPES (Future module: errors.rs)
// =============================================================================

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    Config(String),
}

// =============================================================================
// PERSISTENCE LAYER (Future module: persistence.rs)
// =============================================================================

pub struct PersistenceManager {
    sessions_file: PathBuf,
    config_file: PathBuf,
}

impl PersistenceManager {
    pub fn new() -> Result<Self, PersistenceError> {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("netupi23");

        std::fs::create_dir_all(&data_dir)?;

        Ok(Self {
            sessions_file: data_dir.join("sessions.json"),
            config_file: data_dir.join("config.json"),
        })
    }

    pub async fn save_session(&self, session: &WorkSession) -> Result<(), PersistenceError> {
        let mut sessions = self.load_sessions().await.unwrap_or_default();

        if let Some(pos) = sessions.iter().position(|s| s.id == session.id) {
            sessions[pos] = session.clone();
        } else {
            sessions.push(session.clone());
        }

        let json = serde_json::to_string_pretty(&sessions)?;
        tokio::fs::write(&self.sessions_file, json).await?;
        Ok(())
    }

    pub async fn load_sessions(&self) -> Result<Vec<WorkSession>, PersistenceError> {
        if !self.sessions_file.exists() {
            return Ok(Vec::new());
        }

        let content = tokio::fs::read_to_string(&self.sessions_file).await?;
        let sessions: Vec<WorkSession> = serde_json::from_str(&content)?;
        Ok(sessions) // Fixed: was Ok(())
    }

    pub async fn save_config(&self, config: &TimerConfig) -> Result<(), PersistenceError> {
        let json = serde_json::to_string_pretty(config)?;
        tokio::fs::write(&self.config_file, json).await?;
        Ok(())
    }

    pub async fn load_config(&self) -> Result<TimerConfig, PersistenceError> {
        if !self.config_file.exists() {
            let default_config = TimerConfig::default();
            self.save_config(&default_config).await?;
            return Ok(default_config);
        }

        let content = tokio::fs::read_to_string(&self.config_file).await?;
        let config: TimerConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}

// =============================================================================
// TIMER ENGINE (Future module: timer.rs)
// =============================================================================

pub struct TimerEngine {
    data: Arc<RwLock<TimerData>>,
    config: Arc<RwLock<TimerConfig>>,
    persistence: Arc<PersistenceManager>,
    event_sender: Option<mpsc::UnboundedSender<TimerEvent>>,
    current_session: Arc<RwLock<Option<WorkSession>>>,
}

impl TimerEngine {
    pub async fn new(persistence: Arc<PersistenceManager>) -> Result<Self, PersistenceError> {
        let config = persistence.load_config().await?;

        Ok(Self {
            data: Arc::new(RwLock::new(TimerData::default())),
            config: Arc::new(RwLock::new(config)),
            persistence,
            event_sender: None,
            current_session: Arc::new(RwLock::new(None)),
        })
    }

    pub fn subscribe_events(&mut self) -> mpsc::UnboundedReceiver<TimerEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.event_sender = Some(tx);
        rx
    }

    pub async fn start_timer(&self, timer_type: TimerType) -> Result<(), PersistenceError> {
        self.start_work_session(timer_type, None, None).await
    }

    pub async fn start_work_session(
        &self,
        timer_type: TimerType,
        project_name: Option<String>,
        description: Option<String>,
    ) -> Result<(), PersistenceError> {
        let mut data = self.data.write().await;
        let config = self.config.read().await;

        let target_duration = match timer_type {
            TimerType::PomodoroWork => Some(config.pomodoro_work_duration),
            TimerType::PomodoroShortBreak => Some(config.pomodoro_short_break_duration),
            TimerType::PomodoroLongBreak => Some(config.pomodoro_long_break_duration),
            _ => None,
        };

        *data = TimerData {
            timer_type,
            state: TimerState::Running,
            target_duration,
            elapsed: Duration::zero(),
            paused_time: Duration::zero(),
            start_time: Some(Utc::now()),
            pause_start: None,
        };

        // Create session
        let mut tags = Vec::new();
        if let Some(project) = project_name {
            tags.push(project);
        }

        let session = WorkSession {
            id: uuid::Uuid::new_v4(),
            session_type: timer_type,
            start_time: Utc::now(),
            end_time: None,
            duration: Duration::zero(),
            paused_duration: Duration::zero(),
            description,
            tags,
        };

        *self.current_session.write().await = Some(session);

        if let Some(sender) = &self.event_sender {
            let _ = sender.send(TimerEvent::Started);
        }

        Ok(())
    }

    pub async fn get_current_state(&self) -> TimerData {
        let mut data = self.data.write().await;

        // Update elapsed time if timer is running
        if data.state == TimerState::Running {
            if let Some(start_time) = data.start_time {
                let now = Utc::now();
                data.elapsed = now.signed_duration_since(start_time) - data.paused_time;
            }
        }

        data.clone()
    }

    pub async fn stop_timer(&self) -> Result<(), PersistenceError> {
        let mut data = self.data.write().await;
        data.state = TimerState::Idle;

        // Save current session if exists
        if let Some(session) = self.current_session.write().await.take() {
            let mut final_session = session;
            final_session.end_time = Some(Utc::now());
            final_session.duration = data.elapsed;
            final_session.paused_duration = data.paused_time;

            self.persistence.save_session(&final_session).await?;
        }

        // Reset timer data
        *data = TimerData::default();

        if let Some(sender) = &self.event_sender {
            let _ = sender.send(TimerEvent::Stopped);
        }

        Ok(())
    }
}

// =============================================================================
// DEFAULT IMPLEMENTATIONS (Future module: defaults.rs)
// =============================================================================

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            pomodoro_work_duration: Duration::minutes(25),
            pomodoro_short_break_duration: Duration::minutes(5),
            pomodoro_long_break_duration: Duration::minutes(15),
            long_break_interval: 4,
            auto_start_breaks: false,
            auto_start_work: false,
        }
    }
}

impl Default for TimerData {
    fn default() -> Self {
        Self {
            timer_type: TimerType::Stopwatch,
            state: TimerState::Idle,
            target_duration: None,
            elapsed: Duration::zero(),
            paused_time: Duration::zero(),
            start_time: None,
            pause_start: None,
        }
    }
}

// =============================================================================
// PUBLIC API (Future module: lib.rs only)
// =============================================================================

#[derive(Clone)]
pub struct NetupiCore {
    timer_engine: Arc<TimerEngine>,
}

impl NetupiCore {
    pub async fn new() -> Result<Self, PersistenceError> {
        let persistence = Arc::new(PersistenceManager::new()?);
        let timer_engine = TimerEngine::new(persistence).await?;

        Ok(Self {
            timer_engine: Arc::new(timer_engine),
        })
    }

    pub fn timer(&self) -> &TimerEngine {
        &self.timer_engine
    }

    pub async fn start_work_session(
        &self,
        project_name: String,
        description: Option<String>,
    ) -> Result<(), PersistenceError> {
        self.timer_engine
            .start_work_session(TimerType::Stopwatch, Some(project_name), description)
            .await
    }

    pub async fn get_sessions(&self) -> Result<Vec<WorkSession>, PersistenceError> {
        self.timer_engine.persistence.load_sessions().await
    }

    pub async fn get_projects(&self) -> Result<Vec<(String, Duration)>, PersistenceError> {
        let sessions = self.timer_engine.persistence.load_sessions().await?;
        let mut project_totals: HashMap<String, Duration> = HashMap::new();

        for session in sessions {
            if session.session_type == TimerType::Stopwatch
                && session.end_time.is_some()
                && !session.tags.is_empty()
            {
                let project = session.tags[0].clone();
                let duration = session.duration;
                *project_totals.entry(project).or_insert(Duration::zero()) += duration;
            }
        }

        let mut projects: Vec<(String, Duration)> = project_totals.into_iter().collect();
        projects.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(projects)
    }

    pub async fn get_today_summary(&self) -> Result<HashMap<String, Duration>, PersistenceError> {
        let sessions = self.timer_engine.persistence.load_sessions().await?;
        let today = Utc::now().date_naive();
        let mut project_totals: HashMap<String, Duration> = HashMap::new();

        for session in sessions {
            if let Some(end_time) = session.end_time {
                let session_date = end_time.date_naive();
                if session_date == today
                    && session.session_type == TimerType::Stopwatch
                    && !session.tags.is_empty()
                {
                    let project = session.tags[0].clone();
                    *project_totals.entry(project).or_insert(Duration::zero()) += session.duration;
                }
            }
        }

        Ok(project_totals)
    }
    
    /// Get all completed stopwatch sessions for a specific project (first tag match).
    /// Returns sorted by end_time descending (newest first).
    pub async fn get_sessions_for_project(
        &self,
        project: &str,
    ) -> Result<Vec<WorkSession>, PersistenceError> {
        let sessions = self.timer_engine.persistence.load_sessions().await?;
        let mut project_sessions: Vec<WorkSession> = sessions
            .into_iter()
            .filter(|s| {
                s.session_type == TimerType::Stopwatch
                    && s.end_time.is_some()
                    && !s.tags.is_empty()
                    && s.tags[0] == project
            })
            .collect();
        // Sort by end_time descending (newest first)
        project_sessions.sort_by(|a, b| {
            b.end_time
                .unwrap_or(Utc::now())
                .cmp(&a.end_time.unwrap_or(Utc::now()))
        });
        Ok(project_sessions)
    }

    /// Delete all completed stopwatch sessions for a specific project (first tag match).
    /// Returns the number of sessions deleted.
    pub async fn delete_project_sessions(
        &self,
        project: &str,
    ) -> Result<usize, PersistenceError> {
        let mut sessions = self.timer_engine.persistence.load_sessions().await?;
        let before_len = sessions.len();
        sessions.retain(|s| {
            !(s.session_type == TimerType::Stopwatch
                && s.end_time.is_some()
                && !s.tags.is_empty()
                && s.tags[0] == project)
        });
        let deleted_count = before_len - sessions.len();
        if deleted_count > 0 {
            let json = serde_json::to_string_pretty(&sessions)?;
            tokio::fs::write(
                &self.timer_engine.persistence.sessions_file,
                json,
            )
            .await?;
        }
        Ok(deleted_count)
    }

    
}

// =============================================================================
// TESTS (Future module: tests/ directory)
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_core_creation() {
        let core = NetupiCore::new().await;
        assert!(core.is_ok(), "Core creation should succeed");
    }

    #[tokio::test]
    async fn test_timer_start() {
        let core = NetupiCore::new().await.unwrap();
        let result = core.timer_engine.start_timer(TimerType::PomodoroWork).await;
        assert!(result.is_ok(), "Timer start should succeed");

        let state = core.timer_engine.get_current_state().await;
        assert_eq!(state.state, TimerState::Running);
        assert_eq!(state.timer_type, TimerType::PomodoroWork);
    }

    #[tokio::test]
    async fn test_persistence_manager() {
        let persistence = PersistenceManager::new();
        assert!(
            persistence.is_ok(),
            "PersistenceManager creation should succeed"
        );

        let pm = persistence.unwrap();
        let config = TimerConfig::default();

        let save_result = pm.save_config(&config).await;
        assert!(save_result.is_ok(), "Config save should succeed");

        let load_result = pm.load_config().await;
        assert!(load_result.is_ok(), "Config load should succeed");

        let loaded_config = load_result.unwrap();
        assert_eq!(
            config.pomodoro_work_duration,
            loaded_config.pomodoro_work_duration
        );
    }

    #[tokio::test]
    async fn test_session_persistence() {
        let persistence = PersistenceManager::new().unwrap();

        let session = WorkSession {
            id: uuid::Uuid::new_v4(),
            session_type: TimerType::PomodoroWork,
            start_time: Utc::now(),
            end_time: None,
            duration: Duration::minutes(25),
            paused_duration: Duration::zero(),
            description: Some("Test session".to_string()),
            tags: vec!["test".to_string()],
        };

        let save_result = persistence.save_session(&session).await;
        assert!(save_result.is_ok(), "Session save should succeed");

        let sessions_result = persistence.load_sessions().await;
        assert!(sessions_result.is_ok(), "Sessions load should succeed");

        let sessions = sessions_result.unwrap();
        assert!(!sessions.is_empty(), "Should have at least one session");
        assert_eq!(sessions.last().unwrap().id, session.id);
    }
}
