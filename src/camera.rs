use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;
use std::env::current_dir;
#[cfg(test)]
use std::env::var;
#[cfg(test)]
use std::ffi::OsStr;
use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct JpegOutputSize {
    pub width: usize,
    pub height: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhysicalSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CameraInfo {
    pub id: String,
    pub facing: String,
    pub jpeg_output_sizes: Vec<JpegOutputSize>,
    pub focal_lengths: Vec<f64>,
    pub auto_exposure_modes: Vec<String>,
    pub physical_size: PhysicalSize,
    pub capabilities: Vec<String>,
}

/// Get information about device camera(s).
pub fn info() -> io::Result<Vec<CameraInfo>> {
    let out = run_api_cmd("CameraInfo")?;
    Ok(serde_json::from_str(&out).unwrap())
}

/// Take a photo and save it to a file in JPEG format.
/// 
/// `camera-id` is the id of the camera to use.
/// 
/// See [termux::camera::info]
pub fn photo<P: AsRef<Path>>(camera_id: &str, path: P) -> io::Result<()> {
    let mut p = current_dir()?;
    p.push(path);
    let _out = run_api_cmd_with_args(
        "CameraPhoto",
        &[
            "--es",
            "camera",
            camera_id,
            "--es", "file",
            &p.to_string_lossy()
        ],
    )?;
    Ok(())
}

#[test]
fn test_info() {
    assert!(info().is_ok())
}

#[test]
fn test_photo() {
    let mut p = PathBuf::from(var(OsStr::new("TMPDIR")).unwrap());
    p.push("picture.jpg");
    assert!(photo("0", p).is_ok())
}
