use serde::{Deserialize, Serialize};

pub mod alarm_state;
pub mod alarm_trigger;
pub mod events;

pub const REDIS_EVENTS_LIST_KEY: &str = "events_list";
pub const REDIS_EVENTS_LIST_MAX_ITEMS: u64 = 100;
pub const STATE_PHONE_ALARMS: &str = "phone_alarms";
pub const STATE_PHONE_NOTIFICATIONS: &str = "phone_notifications";
pub const STATE_LOCAL_SIREN: &str = "local_siren";

/// Payload passed along with alarm trigger with
/// information about what was triggered
#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmEvent {
    title: String,
    message: String,
    timestamp: String,
}
