use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CallLog {
    pub name: String,
    pub phone_number: String,
    pub r#type: String,
    pub date: String,
    pub duration: String,
}

/// List call log history.
///
/// The args to the functions are the `offset` and `limit` in call log list respectively.
pub fn log(offset: i32, limit: i32) -> io::Result<Vec<CallLog>> {
    let out = run_api_cmd_with_args(
        "CallLog",
        &[
            "--ei",
            "offset",
            &offset.to_string(),
            "--ei",
            "limit",
            &limit.to_string(),
        ],
    )?;
    Ok(serde_json::from_str(&out).unwrap())
}

#[test]
fn test_log() {
    assert!(log(10, 0).is_ok())
}
