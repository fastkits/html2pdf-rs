use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct VoHtml2pdf {
    pub url: String,
    pub wait_ele: Option<String>,
}
