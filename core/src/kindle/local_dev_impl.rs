use crate::KindleScreensaverError;

pub fn is_kindle_automatic_screensaver_blocked() -> Result<bool, KindleScreensaverError> {
    Ok(false)
}

#[allow(unused)]
pub fn set_kindle_automatic_screensaver_blocked(
    blocked: bool,
) -> Result<(), KindleScreensaverError> {
    Ok(())
}
