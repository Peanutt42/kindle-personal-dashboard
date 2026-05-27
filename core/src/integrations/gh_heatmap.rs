use std::num::ParseIntError;
use std::string::FromUtf8Error;

use chrono::NaiveDate;
use google_tasks1::{
    hyper::{self, Uri, body::Bytes},
    hyper_rustls::{self, HttpsConnector},
    hyper_util::{
        self,
        client::legacy::{Client, Error, connect::HttpConnector},
    },
};
use http_body_util::{BodyExt, combinators::BoxBody};
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FetchGithubContributionsHeatmapError {
    #[error("failed to configure hyper client: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("failed to fetch github contributions heatmap: {0}")]
    HyperUtil(#[from] hyper_util::client::legacy::Error),
    #[error("html response is not valid utf8: {0}")]
    ParseUtf8Response(#[from] FromUtf8Error),
    #[error("failed to extract github contributions heatmap from html: {0}")]
    ParseHtml(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("failed to parse contribution level: {0}")]
    ParseLevel(ParseIntError),
    #[error("failed to parse data-ix: {0}")]
    ParseDataIndex(ParseIntError),
    #[error("failed to parse contribution date: {0}")]
    ParseDate(#[from] chrono::ParseError),
    #[error("cant find table")]
    CantFindTable,
    #[error("Missing {attribute_name} attribute")]
    MissingAttribute { attribute_name: &'static str },
}

/// Contributions for a week. `contribution_levels` can have up to 7 elements
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ContributionWeek {
    /// date of sunday, the first element of `contribution_levels`
    pub sunday_date: NaiveDate,
    /// contribution levels per weekday: sunday, monday, ..., saturday
    pub contribution_levels: Vec<u8>,
}

pub async fn fetch_github_contributions_heatmap(
    username: &str,
) -> Result<Vec<ContributionWeek>, FetchGithubContributionsHeatmapError> {
    let html = fetch_html(
        format!("https://github.com/users/{username}/contributions")
            .parse::<Uri>()
            .unwrap(),
    )
    .await?;

    parse_contribs(&html)
}

async fn fetch_html(uri: Uri) -> Result<String, FetchGithubContributionsHeatmapError> {
    let client: Client<HttpsConnector<HttpConnector>, BoxBody<Bytes, Error>> =
        hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_only()
                .enable_http2()
                .build(),
        );

    let body = client.get(uri).await?;
    Ok(String::from_utf8(
        body.into_body().collect().await?.to_bytes().to_vec(),
    )?)
}

fn parse_contribs(
    html: &str,
) -> Result<Vec<ContributionWeek>, FetchGithubContributionsHeatmapError> {
    let document = Html::parse_document(html);

    let table_selector = Selector::parse("table.ContributionCalendar-grid")?;
    let day_selector = Selector::parse("[data-date]")?;

    let table = document
        .select(&table_selector)
        .next()
        .ok_or(FetchGithubContributionsHeatmapError::CantFindTable)?;

    let mut weeks = vec![None; 53];

    for day in table.select(&day_selector) {
        let value = day.value();

        let get_attribute =
            |attribute_name: &'static str| -> Result<&str, FetchGithubContributionsHeatmapError> {
                value.attr(attribute_name).ok_or(
                    FetchGithubContributionsHeatmapError::MissingAttribute { attribute_name },
                )
            };

        let index_str = get_attribute("data-ix")?;
        let index: usize = index_str
            .parse()
            .map_err(FetchGithubContributionsHeatmapError::ParseDataIndex)?;
        if index >= weeks.len() {
            weeks.resize(index + 1, None);
        }

        let level_str = get_attribute("data-level")?;
        let level: u32 = level_str
            .parse()
            .map_err(FetchGithubContributionsHeatmapError::ParseLevel)?;
        assert!((0..=4).contains(&level));

        let date_str = get_attribute("data-date")?;
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

        weeks[index]
            .get_or_insert_with(|| ContributionWeek {
                sunday_date: date,
                contribution_levels: Vec::new(),
            })
            .contribution_levels
            .push(level as u8);
    }

    Ok(weeks.into_iter().flatten().collect())
}
