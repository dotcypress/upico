use crate::*;
use std::path::Path;
use std::time::Duration;

#[derive(Debug)]
pub enum AppError {
    InvalidLine,
    MountFailed,
    IoError(io::Error),
    ServiceError(io::Error),
    GpioError(gpio_cdev::Error),
    DecodeError(string::FromUtf8Error),
    ProtocolError(rmp_serde::decode::Error),
}

pub type AppResult = Result<(), AppError>;

pub fn mount_pico_dev(disk: &str) -> Result<String, AppError> {
    // Path::/dev/sda1

    let path = Path::new(disk);
    for _ in 0..5 {
        if path.exists() {
            break;
        }
        thread::sleep(Duration::from_millis(1_000));
    }

    for _ in 0..5 {
        if let Ok(output) = process::Command::new("udisksctl")
            .args(["mount", "-b", disk])
            .stdout(process::Stdio::piped())
            .output()
        {
            if output.status.success() {
                let res = String::from_utf8(output.stdout).map_err(AppError::DecodeError)?;
                return res
                    .split(" at ")
                    .last()
                    .map(|s| s.trim().to_owned())
                    .ok_or(AppError::MountFailed);
            }
            thread::sleep(Duration::from_millis(1_000));
        }
    }
    Err(AppError::MountFailed)
}
