use crate::*;

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
    let output = process::Command::new("udisksctl")
        .args(["mount", "-b", disk])
        .stdout(process::Stdio::piped())
        .output()
        .map_err(AppError::IoError)?;

    String::from_utf8(output.stdout)
        .map_err(AppError::DecodeError)?
        .split(" at ")
        .last()
        .map(|s| s.trim().to_owned())
        .ok_or(AppError::MountFailed)
}
