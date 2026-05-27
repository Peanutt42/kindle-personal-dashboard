mod api;

mod state;
pub use state::State;

mod kindle;
pub use kindle::{
    KindleScreensaverError, is_kindle_automatic_screensaver_blocked,
    set_kindle_automatic_screensaver_blocked,
};

mod integrations;
pub use integrations::{ContributionWeek, fetch_github_contributions_heatmap};
