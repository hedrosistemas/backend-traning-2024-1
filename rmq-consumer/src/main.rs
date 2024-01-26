use futures_util::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable, Connection, ConnectionProperties};
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    info!("starting application...");

    info!("starting rabbitmq connection...");

    let addr = format!("amqp://{}:{}@{}:{}", "guest", "guest", "localhost", "5672");

    let Ok(conn) = Connection::connect(&addr, ConnectionProperties::default()).await else {
        error!("rabbitmq connection failure!");
        return Err(());
    };

    info!("rabbitmq connected!");

    info!("starting rabbitmq channel...");

    let Ok(channel) = conn.create_channel().await else {
        error!("rabbitmq channel failure!");
        return Err(());
    };

    let mut consumer = channel
        .basic_consume(
            "batatinha",
            "batatinha-consumer",
            BasicConsumeOptions {
                no_ack: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("failure to create consumer");

    while let Some(event) = consumer.next().await {
        let Ok(delivery) = event else {
            error!("error to receive rmq msg!");
            continue;
        };

        info!("message received!");

        let data = delivery.data;
        info!("{:?}", data);

        info!("message processed successfully!");
    }

    Ok(())
}
