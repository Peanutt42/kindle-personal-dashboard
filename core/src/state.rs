use crate::{
    Config,
    integrations::{
        ContributionWeek, GHHeatmapConfig, fetch_gh_contributions_heatmap,
        get_empty_gh_contributions_heatmap,
    },
    is_kindle_automatic_screensaver_blocked,
};

pub struct State {
    config: Config,
    contribution_weeks: Vec<ContributionWeek>,
}

impl State {
    pub fn new() -> Self {
        let config = match Config::load() {
            Ok(config) => config,
            Err(e) => {
                eprintln!("failed to load config file, using default config: {e}");
                Config::default()
            }
        };

        let is_automatic_screensaver_blocked = is_kindle_automatic_screensaver_blocked();

        println!("is_automatic_screensaver_blocked: {is_automatic_screensaver_blocked:?}");

        let contribution_weeks = Self::fetch_contribution_weeks(config.gh_heatmap.as_ref());

        Self {
            config,
            contribution_weeks,
        }
    }

    pub fn get_contribution_weeks(&self) -> &[ContributionWeek] {
        &self.contribution_weeks
    }

    pub fn refresh(&mut self) {
        println!("Refreshing...");

        self.contribution_weeks = Self::fetch_contribution_weeks(self.config.gh_heatmap.as_ref());
    }

    fn fetch_contribution_weeks(
        gh_heatmap_config: Option<&GHHeatmapConfig>,
    ) -> Vec<ContributionWeek> {
        match gh_heatmap_config {
            Some(config) => tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .enable_io()
                .build()
                .unwrap()
                .block_on(fetch_gh_contributions_heatmap(config))
                .unwrap(),
            None => get_empty_gh_contributions_heatmap(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
