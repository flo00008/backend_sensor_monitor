use std::error::Error;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    results::InsertOneResult,
    Client,
};

use crate::sensors::SensorType;

#[derive(Clone)]
pub struct DbClient {
    client: mongodb::Client,
}

impl DbClient {
    pub async fn new(client_uri: String) -> Result<Self, Box<dyn Error>> {
        // A Client is needed to connect to MongoDB:
        // An extra line of code to work around a DNS issue on Windows:
        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;
        let client = Client::with_options(options)?;

        client
            .list_database_names(None, None)
            .await
            .expect("Unable to connect for database names");

        Ok(DbClient { client })
    }

    pub async fn insert_sensor_data(
        &self,
        sensor: &SensorType,
    ) -> mongodb::error::Result<InsertOneResult> {
        let serialized_sensor = bson::to_bson(&sensor)?;
        let document = serialized_sensor.as_document().unwrap();

        let collection = self.get_collection_for_sensor(&sensor);

        collection.insert_one(document.to_owned(), None).await
    }

    fn get_collection_for_sensor<T>(&self, sensor: &SensorType) -> mongodb::Collection<T> {
        let collection_name = match sensor {
            SensorType::LightSensor(_) => "light_sensor_data",
            SensorType::AccelerometerSensor(_) => "accelerometer_sensor_data",
            SensorType::GeolocationSensor(_) => "geolocation_sensor_data",
            SensorType::OrientationSensor(_) => "orientation_sensor_data",
            SensorType::ProximitySensor(_) => "proximity_sensor_data",
        };

        self.client
            .database("sensors")
            .collection::<T>(collection_name)
    }
}
