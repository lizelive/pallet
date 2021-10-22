
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HealthcheckStatus {
    Starting,
    Healthy,
    Unhealthy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    /// nicely request a restart 
    Restart,
    
    /// Nicely request pause.
    Pause,

    /// Nicely request exit.
    Stop,

    // Kill dead and clean up.
    Remove,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Default)]
pub enum Status {
    /// Starting up
    Created,

    /// Restarting
    Restarting,
    
    /// Running healthy
    Running,

    /// Not currently running
    Paused,

    /// exited and not restartable
    Dead,
    
    /// Stoped
    Exited,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    status: Status,
    pulse: Option<DateTime<Utc>>,
    started: Option<DateTime<Utc>>,
    exited: Option<DateTime<Utc>>,
    health: Option<HealthcheckStatus>,
    exit_code: Option<i64>,
    error: Option<String>,
    id: Option<u64>,
}

impl State {
    pub fn inital() -> Self {
        let pulse = Some(Utc::now());
        Self {
            status: Status::Created,
            pulse,
            started_at: None,
            finished_at: None,
            health: None,
            exit_code: None,
            error: None,
            pid: None,
        }
    }

    /// Set the state's error.
    pub fn add_error(&mut self, error: String) {
        if let Some(old_error) = &self.error {
            self.error = Some(format!("{}; {}", old_error, error));
        } else {
            self.error = Some(error);
        }
    }
}

