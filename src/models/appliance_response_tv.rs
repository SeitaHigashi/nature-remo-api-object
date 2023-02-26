/*
 * Nature API
 *
 * Read/Write Nature Remo
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplianceResponseTv {
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<crate::models::ApplianceResponseLightButtonsInner>>,
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<Vec<crate::models::ApplianceResponseTvLayoutInner>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<crate::models::ApplianceResponseTvState>>,
}

impl ApplianceResponseTv {
    pub fn new() -> ApplianceResponseTv {
        ApplianceResponseTv {
            buttons: None,
            layout: None,
            state: None,
        }
    }
}

