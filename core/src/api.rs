use crate::{
    State, is_kindle_automatic_screensaver_blocked, set_kindle_automatic_screensaver_blocked,
};

#[unsafe(no_mangle)]
pub extern "C" fn kpd_core_state_new() -> *mut State {
    Box::into_raw(Box::new(State::new()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kpd_core_state_get_gh_heatmap_contribution_week_count(
    state: *mut State,
) -> usize {
    assert!(!state.is_null());

    unsafe { state.as_ref().unwrap().get_contribution_weeks().len() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kpd_core_state_get_gh_heatmap_contribution_week_levels(
    state: *mut State,
    week_index: usize,
    out_levels_ptr: *mut *const u8,
    out_levels_count: *mut usize,
) {
    assert!(!state.is_null());
    assert!(!out_levels_ptr.is_null());
    assert!(!out_levels_count.is_null());

    unsafe {
        let weeks = state.as_ref().unwrap().get_contribution_weeks();
        let week = &weeks[week_index];
        *out_levels_count = week.contribution_levels.len();
        *out_levels_ptr = week.contribution_levels.as_ptr();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kpd_core_state_delete(state: *mut State) {
    if !state.is_null() {
        unsafe {
            drop(Box::from_raw(state));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kpd_core_is_automatic_screensaver_blocked() -> bool {
    match is_kindle_automatic_screensaver_blocked() {
        Ok(blocked) => blocked,
        Err(e) => {
            eprintln!("kpd_is_automatic_screensaver_blocked failed: {e}. return false");
            false
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kpd_core_set_automatic_screensaver_blocked(blocked: bool) {
    if let Err(e) = set_kindle_automatic_screensaver_blocked(blocked) {
        eprintln!("kpd_set_automatic_screensaver_blocked failed: {e}");
    }
}
