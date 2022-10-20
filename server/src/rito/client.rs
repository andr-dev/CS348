use rito::{
    apis::{
        configuration::{ApiKey, Configuration},
        summoner_v4_api::{
            summoner_v4_period_get_by_summoner_name, SummonerV4PeriodGetBySummonerNameError,
        },
        Error,
    },
    models::SummonerV4PeriodSummonerDto,
};

pub struct RitoClient {
    config: Configuration,
}

impl RitoClient {
    pub fn new(api_key: ApiKey) -> Self {
        let mut config = Configuration::new();

        config.api_key = Some(api_key);

        Self { config }
    }

    pub async fn get_by_summoner_name(
        &self,
        name: String,
    ) -> Result<SummonerV4PeriodSummonerDto, Error<SummonerV4PeriodGetBySummonerNameError>> {
        summoner_v4_period_get_by_summoner_name(&self.config, &name).await
    }
}
