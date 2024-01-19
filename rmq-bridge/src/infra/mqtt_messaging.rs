use futures_util::StreamExt;
use log::{error, info};
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder};
use std::env::var;

pub struct MQTTMessaging {}

impl MQTTMessaging {
    pub async fn new() -> Result<Self, ()> {
        let Ok(mqtt_host) = var("MQTT_HOST") else {
            error!("failure to read the MQTT_HOST env");
            return Err(());
        };

        let Ok(mqtt_protocol) = var("MQTT_PROTOCOL") else {
            error!("failure to read the MQTT_PROTOCOL env");
            return Err(());
        };

        let configs = CreateOptionsBuilder::new()
            .server_uri(format!("{}://{}", mqtt_protocol, mqtt_host))
            .finalize();

        let Ok(mut client) = AsyncClient::new(configs) else {
            error!("failure to create MQTT AsyncClient instance");
            return Err(());
        };

        let conn_opts = ConnectOptionsBuilder::new().finalize();

        match client.connect(Some(conn_opts)).await {
            Ok(_) => {
                client.subscribe("HedroTraining2024/#", 2);

                let mut stream = client.get_stream(2048);

                while let Some(opt_infos) = stream.next().await {
                    if let Some(infos) = opt_infos {
                        info!("received message {}", infos.topic())
                    }
                }

                Ok(Self {})
            }
            Err(_) => {
                error!("MQTT failure to connect!");
                Err(())
            }
        }
    }
}

impl MQTTMessaging {}

impl MQTTMessaging {
    pub fn ooo(&self) {}
}
