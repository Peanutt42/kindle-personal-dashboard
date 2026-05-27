use crate::{
    ContributionWeek, integrations::fetch_github_contributions_heatmap,
    is_kindle_automatic_screensaver_blocked,
};

pub struct State {
    contribution_weeks: Vec<ContributionWeek>,
}

impl State {
    pub fn new() -> Self {
        let is_automatic_screensaver_blocked = is_kindle_automatic_screensaver_blocked();

        let contribution_weeks = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()
            .unwrap()
            .block_on(fetch_github_contributions_heatmap("Peanutt42"))
            .unwrap();

        println!("is_automatic_screensaver_blocked: {is_automatic_screensaver_blocked:?}");

        Self { contribution_weeks }
    }

    pub fn get_contribution_weeks(&self) -> &[ContributionWeek] {
        &self.contribution_weeks
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
