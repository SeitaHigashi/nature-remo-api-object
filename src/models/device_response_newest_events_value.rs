/*
 * Nature API
 *
 * Read/Write Nature Remo
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeviceResponseNewestEventsValue : The SensorValue key means 'te' = temperature, 'hu' = humidity, 'il' = illumination, 'mo' = movement. The val of 'mo' is always 1 and when movement event is captured created_at is updated.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeviceResponseNewestEventsValue {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "val", skip_serializing_if = "Option::is_none")]
    pub val: Option<f32>,
}

impl DeviceResponseNewestEventsValue {
    /// The SensorValue key means 'te' = temperature, 'hu' = humidity, 'il' = illumination, 'mo' = movement. The val of 'mo' is always 1 and when movement event is captured created_at is updated.
    pub fn new() -> DeviceResponseNewestEventsValue {
        DeviceResponseNewestEventsValue {
            created_at: None,
            val: None,
        }
    }
}

