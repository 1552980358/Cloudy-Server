use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

type Result<T> = std::result::Result<T, SystemTimeError>;

fn system_time_now() -> Result<Duration> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
}

pub fn system_time_secs() -> Result<u64> {
    system_time_now().map(|duration| duration.as_secs())
}