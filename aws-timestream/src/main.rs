use std::error::Error;

use aws_sdk_timestreamwrite::{
    self,
    types::{Dimension, DimensionValueType, MeasureValue, MeasureValueType, Record, TimeUnit},
};

#[::tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = aws_config::load_from_env().await;

    let client = match aws_sdk_timestreamwrite::Client::new(&config)
        .with_endpoint_discovery_enabled()
        .await
    {
        Ok((c, _)) => Ok(c),
        Err(err) => {
            println!("failure to connect");
            Err(err)
        }
    }?;

    let measure = MeasureValue::builder()
        .set_name(Some("temp".into()))
        .set_type(Some(MeasureValueType::Double))
        .set_value(Some("10.5".into()))
        .build()
        .expect("failed to create MeasureValue");

    let dimension = Dimension::builder()
        .set_name(Some("device".into()))
        .set_dimension_value_type(Some(DimensionValueType::Varchar))
        .set_value(Some("0001".into()))
        .build()
        .expect("failed to create Dimension");

    let record = Record::builder()
        .set_time(Some("1706623485234".into()))
        .set_time_unit(Some(TimeUnit::Milliseconds))
        .set_measure_name(Some("mesure-name".into()))
        .set_measure_values(Some(vec![measure]))
        .set_measure_value_type(Some(MeasureValueType::Multi))
        .set_dimensions(Some(vec![dimension]))
        .build();

    match client
        .write_records()
        .set_database_name(Some("hdr-training".into()))
        .set_table_name(Some("ralvescosta".to_string()))
        .set_records(Some(vec![record]))
        .send()
        .await
    {
        Ok(_) => {
            println!("inserted!");
        }
        Err(err) => {
            println!("failed to insert values");
            println!("{:?}", err);
        }
    }

    Ok(())
}
