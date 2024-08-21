use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BatteryStatus {
    pub health: String,
    pub percentage: u8,
    pub plugged: String,
    pub status: String,
    pub temperature: f64,
    pub current: i32,
}

/// Get the status of the device battery.
pub fn status() -> io::Result<BatteryStatus> {
    let out = run_api_cmd("BatteryStatus")?;
    Ok(serde_json::from_str(&out).unwrap())
}

#[test]
fn test_status() {
    assert!(status().is_ok())
}
