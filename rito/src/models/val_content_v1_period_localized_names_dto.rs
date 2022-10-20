#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValContentV1PeriodLocalizedNamesDto {
    #[serde(rename = "ar-AE")]
    pub ar_ae: String,
    #[serde(rename = "de-DE")]
    pub de_de: String,
    #[serde(rename = "en-GB", skip_serializing_if = "Option::is_none")]
    pub en_gb: Option<String>,
    #[serde(rename = "en-US")]
    pub en_us: String,
    #[serde(rename = "es-ES")]
    pub es_es: String,
    #[serde(rename = "es-MX")]
    pub es_mx: String,
    #[serde(rename = "fr-FR")]
    pub fr_fr: String,
    #[serde(rename = "id-ID")]
    pub id_id: String,
    #[serde(rename = "it-IT")]
    pub it_it: String,
    #[serde(rename = "ja-JP")]
    pub ja_jp: String,
    #[serde(rename = "ko-KR")]
    pub ko_kr: String,
    #[serde(rename = "pl-PL")]
    pub pl_pl: String,
    #[serde(rename = "pt-BR")]
    pub pt_br: String,
    #[serde(rename = "ru-RU")]
    pub ru_ru: String,
    #[serde(rename = "th-TH")]
    pub th_th: String,
    #[serde(rename = "tr-TR")]
    pub tr_tr: String,
    #[serde(rename = "vi-VN")]
    pub vi_vn: String,
    #[serde(rename = "zh-CN")]
    pub zh_cn: String,
    #[serde(rename = "zh-TW")]
    pub zh_tw: String,
}

impl ValContentV1PeriodLocalizedNamesDto {
    pub fn new(
        ar_ae: String,
        de_de: String,
        en_us: String,
        es_es: String,
        es_mx: String,
        fr_fr: String,
        id_id: String,
        it_it: String,
        ja_jp: String,
        ko_kr: String,
        pl_pl: String,
        pt_br: String,
        ru_ru: String,
        th_th: String,
        tr_tr: String,
        vi_vn: String,
        zh_cn: String,
        zh_tw: String,
    ) -> ValContentV1PeriodLocalizedNamesDto {
        ValContentV1PeriodLocalizedNamesDto {
            ar_ae,
            de_de,
            en_gb: None,
            en_us,
            es_es,
            es_mx,
            fr_fr,
            id_id,
            it_it,
            ja_jp,
            ko_kr,
            pl_pl,
            pt_br,
            ru_ru,
            th_th,
            tr_tr,
            vi_vn,
            zh_cn,
            zh_tw,
        }
    }
}
