


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`champion_mastery_v4_period_get_all_champion_masteries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChampionMasteryV4PeriodGetAllChampionMasteriesError {
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

/// struct for typed errors of method [`champion_mastery_v4_period_get_champion_mastery`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChampionMasteryV4PeriodGetChampionMasteryError {
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

/// struct for typed errors of method [`champion_mastery_v4_period_get_champion_mastery_score`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChampionMasteryV4PeriodGetChampionMasteryScoreError {
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

/// struct for typed errors of method [`champion_mastery_v4_period_get_top_champion_masteries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChampionMasteryV4PeriodGetTopChampionMasteriesError {
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


/// Get all champion mastery entries sorted by number of champion points descending,
pub async fn champion_mastery_v4_period_get_all_champion_masteries(configuration: &configuration::Configuration, encrypted_summoner_id: &str) -> Result<Vec<crate::models::ChampionMasteryV4PeriodChampionMasteryDto>, Error<ChampionMasteryV4PeriodGetAllChampionMasteriesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lol/champion-mastery/v4/champion-masteries/by-summoner/{encryptedSummonerId}", local_var_configuration.base_path, encryptedSummonerId=crate::apis::urlencode(encrypted_summoner_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("api_key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<ChampionMasteryV4PeriodGetAllChampionMasteriesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a champion mastery by player ID and champion ID.
pub async fn champion_mastery_v4_period_get_champion_mastery(configuration: &configuration::Configuration, champion_id: i64, encrypted_summoner_id: &str) -> Result<crate::models::ChampionMasteryV4PeriodChampionMasteryDto, Error<ChampionMasteryV4PeriodGetChampionMasteryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lol/champion-mastery/v4/champion-masteries/by-summoner/{encryptedSummonerId}/by-champion/{championId}", local_var_configuration.base_path, championId=champion_id, encryptedSummonerId=crate::apis::urlencode(encrypted_summoner_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("api_key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<ChampionMasteryV4PeriodGetChampionMasteryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a player's total champion mastery score, which is the sum of individual champion mastery levels.
pub async fn champion_mastery_v4_period_get_champion_mastery_score(configuration: &configuration::Configuration, encrypted_summoner_id: &str) -> Result<i32, Error<ChampionMasteryV4PeriodGetChampionMasteryScoreError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lol/champion-mastery/v4/scores/by-summoner/{encryptedSummonerId}", local_var_configuration.base_path, encryptedSummonerId=crate::apis::urlencode(encrypted_summoner_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("api_key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<ChampionMasteryV4PeriodGetChampionMasteryScoreError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get specified number of top champion mastery entries sorted by number of champion points descending.
pub async fn champion_mastery_v4_period_get_top_champion_masteries(configuration: &configuration::Configuration, encrypted_summoner_id: &str, count: Option<i32>) -> Result<Vec<crate::models::ChampionMasteryV4PeriodChampionMasteryDto>, Error<ChampionMasteryV4PeriodGetTopChampionMasteriesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/lol/champion-mastery/v4/champion-masteries/by-summoner/{encryptedSummonerId}/top", local_var_configuration.base_path, encryptedSummonerId=crate::apis::urlencode(encrypted_summoner_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
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
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<ChampionMasteryV4PeriodGetTopChampionMasteriesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

