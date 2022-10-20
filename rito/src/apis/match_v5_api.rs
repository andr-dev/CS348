use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`match_v5_period_get_match`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchV5PeriodGetMatchError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status405(),
    Status415(),
    Status429(),
    Status500(),
    Status502(),
    Status503(),
    Status504(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`match_v5_period_get_match_ids_by_puuid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchV5PeriodGetMatchIdsByPuuidError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status405(),
    Status415(),
    Status429(),
    Status500(),
    Status502(),
    Status503(),
    Status504(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`match_v5_period_get_timeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchV5PeriodGetTimelineError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status405(),
    Status415(),
    Status429(),
    Status500(),
    Status502(),
    Status503(),
    Status504(),
    UnknownValue(serde_json::Value),
}

/// Get a match by match id
pub async fn match_v5_period_get_match(
    configuration: &configuration::Configuration,
    match_id: &str,
) -> Result<crate::models::MatchV5PeriodMatchDto, Error<MatchV5PeriodGetMatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/lol/match/v5/matches/{matchId}",
        local_var_configuration.base_path,
        matchId = crate::apis::urlencode(match_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("api_key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Riot-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MatchV5PeriodGetMatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of match ids by puuid
pub async fn match_v5_period_get_match_ids_by_puuid(
    configuration: &configuration::Configuration,
    puuid: &str,
    start_time: Option<i64>,
    end_time: Option<i64>,
    queue: Option<i32>,
    r#type: Option<&str>,
    start: Option<i32>,
    count: Option<i32>,
) -> Result<Vec<String>, Error<MatchV5PeriodGetMatchIdsByPuuidError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/lol/match/v5/matches/by-puuid/{puuid}/ids",
        local_var_configuration.base_path,
        puuid = crate::apis::urlencode(puuid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start_time {
        local_var_req_builder =
            local_var_req_builder.query(&[("startTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder =
            local_var_req_builder.query(&[("endTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = queue {
        local_var_req_builder =
            local_var_req_builder.query(&[("queue", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = count {
        local_var_req_builder =
            local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("api_key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Riot-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MatchV5PeriodGetMatchIdsByPuuidError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a match timeline by match id
pub async fn match_v5_period_get_timeline(
    configuration: &configuration::Configuration,
    match_id: &str,
) -> Result<crate::models::MatchV5PeriodMatchTimelineDto, Error<MatchV5PeriodGetTimelineError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/lol/match/v5/matches/{matchId}/timeline",
        local_var_configuration.base_path,
        matchId = crate::apis::urlencode(match_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("api_key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Riot-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MatchV5PeriodGetTimelineError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
