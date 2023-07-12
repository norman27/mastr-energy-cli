use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize};

#[derive(Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "Data")]
    pub data: Vec<Unit>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Unit {
    #[serde(rename = "Id")]
    pub id: u32,

    #[serde(rename = "BetriebsStatusId")]
    pub activation_id: u16,

    #[serde(rename = "AnzahlSolarModule")]
    pub pv_module_count: Option<u32>,

    #[serde(rename = "Bruttoleistung")]
    pub gross_power: f32,

    #[serde(rename = "Nettonennleistung")]
    pub net_power: f32,

    #[serde(rename = "EnergietraegerName")]
    pub energy_carrier: String,

    #[serde(rename = "Typ")]
    pub category: u32,

    #[serde(rename = "InbetriebnahmeDatum", deserialize_with = "date_serializer")] //@TODO custom serializer
    pub start_datetime_ms: Option<i64>,
}

fn date_serializer<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Option<String> = serde::Deserialize::deserialize(deserializer)?;
    match raw {
        Some(raw_str) => {
            let date_str = raw_str.replace("/Date(", "").replace(")/", "");
            let date_i64 = date_str.parse::<i64>().map_err(D::Error::custom)?;
            Ok(Some(date_i64))
        }
        None => Ok(None),
    }
}

impl Unit {
    pub fn is_active(&self) -> bool {
        self.activation_id == 35
    }

    pub fn is_solar(&self) -> bool {
        self.category == 1
    }

    pub fn is_balcony(&self) -> bool {
        // @TODO needs refinement regarding 0.8
        return self.net_power <= 0.6 && self.pv_module_count <= Some(2);
    }

    pub fn start_ymd(&self) -> i32 {
        let mut time = NaiveDateTime::from_timestamp_millis(0);

        match self.start_datetime_ms {
            Some(s) => time = NaiveDateTime::from_timestamp_millis(s),
            None => (),
        }


        return time.unwrap().format("%Y%m%d").to_string().parse::<i32>().unwrap();
    }
}
