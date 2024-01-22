mod infra;
mod services;

use crate::{infra::mqtt_messaging::MQTTMessaging, services::service::BridgeServiceImpl};
use log::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failure to read .env");
    env_logger::init();

    info!("starting application...");

    let service = BridgeServiceImpl::new();

    let mut messaging = MQTTMessaging::new(Box::new(service));

    messaging.subscribe("HedroTraining2024/#".into(), 2);

    messaging
        .connect()
        .await
        .expect("failure to connect to MQTT");

    info!("MQTT connected!");
}
