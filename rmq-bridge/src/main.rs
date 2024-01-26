mod infra;
mod services;

use crate::{
    infra::{mqtt_messaging::MQTTMessaging, rmq_messaging::RabbitMQMessaging},
    services::service::BridgeServiceImpl,
};
use log::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failure to read .env");
    env_logger::init();

    info!("starting application...");

    let mut rmq_messaging = RabbitMQMessaging::new();
    rmq_messaging
        .connect()
        .await
        .expect("rabbitmq connection failure!");

    let service = BridgeServiceImpl::new(Box::new(rmq_messaging));

    let mut mqtt_messaging = MQTTMessaging::new(Box::new(service));

    mqtt_messaging.subscribe("HedroTraining2024/#".into(), 2);

    mqtt_messaging
        .connect()
        .await
        .expect("failure to connect to MQTT");

    info!("MQTT connected!");
}
