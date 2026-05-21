use thiserror::Error;

#[derive(Debug, Error)]
pub enum KindleScreensaverError {
    #[error(
        "failed to query if screensaver is enabled because of unknown output of {cmd}: stdout={stdout_output}, stderr={stderr_output}"
    )]
    UnknownOutputOfLipcGetProp {
        cmd: String,
        stdout_output: String,
        stderr_output: String,
    },
    #[error(
		"{cmd} exited unsuccessfully with exit code {} while {while_doing}",
		exit_code.as_ref().map(i32::to_string).unwrap_or("unknown".to_string())
	)]
    ProcessExitedUnsuccessfully {
        cmd: String,
        while_doing: String,
        exit_code: Option<i32>,
    },
    #[error("{cmd} failed to run while {while_doing}: {cause}")]
    ProgramFailedToRun {
        cmd: String,
        while_doing: String,
        cause: std::io::Error,
    },
}

#[cfg(not(feature = "local_dev"))]
mod device_impl;
#[cfg(not(feature = "local_dev"))]
pub use device_impl::{
    is_kindle_automatic_screensaver_blocked, set_kindle_automatic_screensaver_blocked,
};

#[cfg(feature = "local_dev")]
mod local_dev_impl;
#[cfg(feature = "local_dev")]
pub use local_dev_impl::{
    is_kindle_automatic_screensaver_blocked, set_kindle_automatic_screensaver_blocked,
};
