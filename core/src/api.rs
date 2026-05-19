use crate::{
    State, is_kindle_automatic_screensaver_blocked, set_kindle_automatic_screensaver_blocked,
};

#[unsafe(no_mangle)]
pub extern "C" fn kpd_state_new() -> *mut State {
    Box::into_raw(Box::new(State::new()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kpd_state_delete(state: *mut State) {
    if !state.is_null() {
        unsafe {
            drop(Box::from_raw(state));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kpd_is_automatic_screensaver_blocked() -> bool {
    match is_kindle_automatic_screensaver_blocked() {
        Ok(blocked) => blocked,
        Err(e) => {
            eprintln!("kpd_is_automatic_screensaver_blocked failed: {e}. return false");
            false
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kpd_set_automatic_screensaver_blocked(blocked: bool) {
    if let Err(e) = set_kindle_automatic_screensaver_blocked(blocked) {
        eprintln!("kpd_set_automatic_screensaver_blocked failed: {e}");
    }
}
