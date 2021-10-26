use chrono::{DateTime, Utc};
pub type Timestamp = DateTime<Utc>;

use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
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

impl Default for Status {
    fn default() -> Self {
        Self::Created
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct State {
    status: Status,
    pulse: Option<Timestamp>,
    started: Option<Timestamp>,
    exited: Option<Timestamp>,
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
            started: None,
            exited: None,
            health: None,
            exit_code: None,
            error: None,
            id: None,
        }
    }

    /// helper function to add an error.
    pub fn add_error(&mut self, error: String) {
        if let Some(old_error) = &self.error {
            self.error = Some(format!("{}; {}", old_error, error));
        } else {
            self.error = Some(error);
        }
    }
}
