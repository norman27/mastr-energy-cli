use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "Data")]
    pub data: Vec<Unit>,
}

#[derive(Clone,Debug,Deserialize)]
pub struct Unit {
    #[serde(rename = "Id")]
    pub id: u32,

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
