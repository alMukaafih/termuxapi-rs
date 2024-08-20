use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct AudioInfo {
    pub property_output_sample_rate: String,
    pub property_output_frames_per_buffer: String,
    pub audiotrack_sample_rate: i32,
    pub audiotrack_buffer_size_in_frames: i32,
    pub audiotrack_sample_rate_low_latency: i32,
    pub bluetooth_a2dp_is_on: bool,
    pub wiredheadset_is_connected: bool,
}

/// Get information about audio capabilities.
pub fn info() -> io::Result<AudioInfo> {
    let out = run_api_cmd("AudioInfo")?;
    let i: AudioInfo = serde_json::from_str(&out).unwrap();
    Ok(i)
}

#[test]
fn test_info() {
    assert!(info().is_ok())
}