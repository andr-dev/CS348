use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod account_v1_api;
pub mod champion_mastery_v4_api;
pub mod champion_v3_api;
pub mod clash_v1_api;
pub mod league_exp_v4_api;
pub mod league_v4_api;
pub mod lol_challenges_v1_api;
pub mod lol_status_v3_api;
pub mod lol_status_v4_api;
pub mod lor_deck_v1_api;
pub mod lor_inventory_v1_api;
pub mod lor_match_v1_api;
pub mod lor_ranked_v1_api;
pub mod lor_status_v1_api;
pub mod match_v5_api;
pub mod spectator_v4_api;
pub mod summoner_v4_api;
pub mod tft_league_v1_api;
pub mod tft_match_v1_api;
pub mod tft_status_v1_api;
pub mod tft_summoner_v1_api;
pub mod tournament_stub_v4_api;
pub mod tournament_v4_api;
pub mod val_content_v1_api;
pub mod val_match_v1_api;
pub mod val_ranked_v1_api;
pub mod val_status_v1_api;

pub mod configuration;
