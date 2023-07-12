use serde::Deserialize;

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

    #[serde(rename = "EnergietraegerName")]
    pub energy_carrier: String,

    #[serde(rename = "Typ")]
    pub category: u32,

    #[serde(rename = "InbetriebnahmeDatum")] //@TODO custom serializer
    pub start_datetime: Option<String>,
}

impl Unit {
    pub fn is_active(&self) -> bool {
        self.activation_id == 35 // 35 = In Betrieb
    }

    pub fn is_solar(&self) -> bool {
        self.category == 1 // 1 = Solare Strahlungsenergie
    }
}
