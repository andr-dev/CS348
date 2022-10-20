use rito::{
    apis::{
        configuration::{ApiKey, Configuration},
        ddragon_champion::{ddragon_champion_get_champions, DDragonChampionResponseError},
        match_v5_api::{
            match_v5_period_get_match, match_v5_period_get_match_ids_by_puuid,
            MatchV5PeriodGetMatchError, MatchV5PeriodGetMatchIdsByPuuidError,
        },
        summoner_v4_api::{
            summoner_v4_period_get_by_puuid, summoner_v4_period_get_by_summoner_name,
            SummonerV4PeriodGetByPuuidError, SummonerV4PeriodGetBySummonerNameError,
        },
        Error,
    },
    models::{
        DDragonChampionResponse, MatchV5PeriodMatchDto, MatchV5PeriodParticipantDto,
        SummonerV4PeriodSummonerDto,
    },
};

pub struct RitoClient {
    config_zone: Configuration,   // NA, BR, ...
    config_region: Configuration, // AMERICAS
    config_ddragon: Configuration,
}

impl RitoClient {
    pub fn new(api_key: ApiKey) -> Self {
        let (mut config_zone, mut config_region, mut config_ddragon) = (
            Configuration::new(),
            Configuration::new(),
            Configuration::new(),
        );

        config_zone.api_key = Some(api_key.clone());
        config_region.api_key = Some(api_key.clone());
        config_ddragon.api_key = Some(api_key);

        config_zone.base_path = "https://na1.api.riotgames.com".to_string();
        config_ddragon.base_path = "http://ddragon.leagueoflegends.com".to_string();

        Self {
            config_zone,
            config_region,
            config_ddragon,
        }
    }

    pub async fn get_summoner_by_summoner_puuid(
        &self,
        puuid: &str,
    ) -> Result<SummonerV4PeriodSummonerDto, Error<SummonerV4PeriodGetByPuuidError>> {
        summoner_v4_period_get_by_puuid(&self.config_zone, puuid).await
    }

    pub async fn get_summoner_by_summoner_name(
        &self,
        name: &str,
    ) -> Result<SummonerV4PeriodSummonerDto, Error<SummonerV4PeriodGetBySummonerNameError>> {
        summoner_v4_period_get_by_summoner_name(&self.config_zone, name).await
    }

    pub async fn get_match_ids_by_puuid(
        &self,
        puuid: &str,
    ) -> Result<Vec<String>, Error<MatchV5PeriodGetMatchIdsByPuuidError>> {
        match_v5_period_get_match_ids_by_puuid(
            &self.config_region,
            puuid,
            None,
            None,
            None,
            None,
            Some(0),
            Some(5),
        )
        .await
    }

    pub async fn get_match_by_match_id(
        &self,
        match_id: &str,
    ) -> Result<MatchV5PeriodMatchDto, Error<MatchV5PeriodGetMatchError>> {
        match_v5_period_get_match(&self.config_region, match_id).await
    }

    pub async fn get_champions(
        &self,
    ) -> Result<DDragonChampionResponse, Error<DDragonChampionResponseError>> {
        ddragon_champion_get_champions(&self.config_ddragon).await
    }
}
