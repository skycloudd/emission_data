use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MeansOfTransport {
    #[serde(rename = "odata.metadata")]
    pub metadata_url: String,

    #[serde(rename = "value")]
    pub data: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "CategoryGroupID")]
    pub category_group_id: usize,
}
