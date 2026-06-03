mod api;

mod state;
pub use state::State;

mod config;
pub use config::Config;

mod kindle;
pub use kindle::{
    KindleScreensaverError, is_kindle_automatic_screensaver_blocked,
    set_kindle_automatic_screensaver_blocked,
};

mod integrations;
