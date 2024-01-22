use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MQTTMessage {
    pub device: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typ: String,
    pub value: String,
}
