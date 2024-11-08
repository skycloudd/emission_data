use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TypedDataset {
    #[serde(rename = "odata.metadata")]
    pub metadata_url: String,

    #[serde(rename = "value")]
    pub data: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    #[serde(rename = "ID")]
    pub id: usize,

    #[serde(rename = "MeansOfTransport")]
    pub means_of_transport: String,

    #[serde(rename = "Periods")]
    pub periods: String,

    #[serde(rename = "CarbonDioxideCO2_1")]
    pub carbon_dioxide_co2_1: f64,
}
