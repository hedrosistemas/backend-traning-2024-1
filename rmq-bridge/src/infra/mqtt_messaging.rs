use futures_util::StreamExt;
use log::{error, info};
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, Message};
use std::env::var;

use crate::services::{messages::MQTTMessage, service::BridgeService};

struct MQTTConfigs {
    protocol: String,
    host: String,
    port: String,
    client_id: String,
}

pub struct MQTTMessaging {
    subscribes: Vec<(String, u8)>,
    service: Box<dyn BridgeService>,
}

impl MQTTMessaging {
    pub fn new(service: Box<dyn BridgeService>) -> Self {
        MQTTMessaging {
            subscribes: vec![],
            service,
        }
    }
}

impl MQTTMessaging {
    fn envs(&self) -> Result<MQTTConfigs, ()> {
        let Ok(host) = var("MQTT_HOST") else {
            error!("failure to read the MQTT_HOST env");
            return Err(());
        };

        let Ok(port) = var("MQTT_PORT") else {
            error!("failure to read the MQTT_PORT env");
            return Err(());
        };

        let Ok(protocol) = var("MQTT_PROTOCOL") else {
            error!("failure to read the MQTT_PROTOCOL env");
            return Err(());
        };

        let Ok(client_id) = var("MQTT_CLIENT_ID") else {
            error!("failure to read the MQTT_CLIENT_ID env");
            return Err(());
        };

        Ok(MQTTConfigs {
            client_id,
            host,
            port,
            protocol,
        })
    }

    pub async fn connect(&self) -> Result<(), ()> {
        let env = self.envs()?;

        let configs = CreateOptionsBuilder::new()
            .server_uri(format!("{}://{}:{}", env.protocol, env.host, env.port))
            .client_id(env.client_id)
            .finalize();

        let Ok(mut client) = AsyncClient::new(configs) else {
            error!("failure to create MQTT AsyncClient instance");
            return Err(());
        };

        let conn_opts = ConnectOptionsBuilder::new().finalize();

        let Ok(_) = client.connect(Some(conn_opts)).await else {
            error!("failure to connect to MQTT");
            return Err(());
        };

        for (topic, qos) in self.subscribes.clone() {
            client.subscribe(topic, qos.into());
        }

        let mut stream = client.get_stream(2048);
        while let Some(opt_infos) = stream.next().await {
            self.handler(opt_infos);
        }

        Ok(())
    }

    pub fn subscribe(&mut self, topic: String, qos: u8) {
        self.subscribes.push((topic, qos));
    }

    async fn handler(&self, infos: Option<Message>) {
        let Some(message) = infos else {
            return;
        };

        let payload = message.payload();

        let Ok(msg) = serde_json::from_slice::<MQTTMessage>(payload) else {
            error!("failed to deserialized message!");
            return;
        };

        match self.service.exec(&msg).await {
            Ok(_) => {
                info!("message processed successfully!")
            }
            Err(_) => error!("failed to process message"),
        }
    }
}
