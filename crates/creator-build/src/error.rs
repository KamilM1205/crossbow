use displaydoc::Display;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum AndroidError {
    /// Android SDK is not found
    AndroidSdkNotFound,
    /// Android NDK is not found
    AndroidNdkNotFound,
    /// Android SDK has no build tools
    BuildToolsNotFound,
    /// Android SDK has no platforms installed
    NoPlatformsFound,
    /// Platform {0} is not installed
    PlatformNotFound(u32),
    /// Target is not supported
    UnsupportedTarget,
    /// Host {0} is not supported
    UnsupportedHost(String),
}

#[derive(Display, Debug, Error)]
pub enum AppleError {
    /// Plist data error
    Plist(#[from] plist::Error),
}

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Command '{0:?}' had a non-zero exit code
    CmdFailed(Command),
    /// Command {0} not found
    CmdNotFound(String),
    /// Path {0:?} doesn't exist
    PathNotFound(PathBuf),
    /// IO error
    Io(#[from] std::io::Error),
    /// Android error
    Android(#[from] AndroidError),
    /// Apple error
    Apple(#[from] AppleError),
    /// Other error
    OtherError(#[from] Box<dyn std::error::Error>),
}

impl From<plist::Error> for Error {
    fn from(error: plist::Error) -> Self {
        AppleError::from(error).into()
    }
}
