use std::time::Duration;

#[derive(Clone, Debug)]
pub struct SortStats {
    total_steps: u64,
    duration: Duration,
}

#[derive(Clone, Debug)]
pub struct StatsSnapshot {
    pub total_steps: u64,
    pub duration_seconds: f64,
    pub duration_milliseconds: f64,
}

impl SortStats {
    pub fn from_measurements(total_steps: u64, duration: Duration) -> Self {
        Self { total_steps, duration }
    }

    pub fn to_snapshot(&self) -> StatsSnapshot {
        let duration_seconds = self.duration.as_secs_f64();
        let duration_milliseconds = duration_seconds * 1000.0;

        StatsSnapshot { 
            total_steps: self.total_steps,
            duration_seconds,
            duration_milliseconds,
        }
    }
}