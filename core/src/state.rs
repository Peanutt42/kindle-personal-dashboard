use crate::is_kindle_automatic_screensaver_blocked;

pub struct State {}

impl State {
    pub fn new() -> Self {
        let is_automatic_screensaver_blocked = is_kindle_automatic_screensaver_blocked();

        println!("is_automatic_screensaver_blocked: {is_automatic_screensaver_blocked:?}");

        Self {}
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
