mod infra;
mod services;

use crate::infra::mqtt_messaging::MQTTMessaging;
use log::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failure to read .env");
    env_logger::init();

    info!("starting application...");

    MQTTMessaging::new().await.expect("MQTT connection failure");

    info!("MQTT connected!");
}

// [x] - Array e vector
// [x] - Pattern Matching
// [x] - Struct like Rust
// [x] - Enum like Rust
// [x] - Pointer e Smart Pointer
// [x] - Trait
// [x] - Rust Generics
// [x] - Macros
// [x] - Annotations
// [x] - Async Rust
