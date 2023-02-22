use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Accelerometer {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    x: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    y: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    z: f32,

    #[serde(flatten)]
    base: SensorBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    #[serde(rename = "Light")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    light: f32,

    #[serde(flatten)]
    base: SensorBase,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorType {
    LightSensor(Light),
    AccelerometerSensor(Accelerometer),
}

#[derive(Debug, Serialize, Deserialize)]
struct SensorBase {
    #[serde(rename = "SensorName")]
    sensor_name: String,
    #[serde(rename = "Timestamp")]
    timestamp: u64,
    payload: String,
}

#[cfg(test)]
mod tests {
    use crate::sensors::*;

    #[test]
    fn deserialize_light_sensor() -> Result<(), serde_json::Error> {
        let json = "{\"SensorName\":\"LightSensor\",\"Timestamp\":1677023344081,\"Light\":\"17\",\"payload\":\"\"}";

        let _: Light = serde_json::from_str(json)?;

        let s: SensorType = serde_json::from_str(json)?;

        assert!(matches!(s, SensorType::LightSensor(_)));

        Ok(())
    }

    #[test]
    fn deserialize_accelerometer_sensor() -> Result<(), serde_json::Error> {
        let json = "{\"SensorName\":\"Accelerometer\",\"Timestamp\":1677025378441,\"x\":\"-3.0370447635650635\",\"y\":\"8.360552787780762\",\"z\":\"4.320336818695068\",\"payload\":\"\"}";

        let _: Accelerometer = serde_json::from_str(json)?;

        let s: SensorType = serde_json::from_str(json)?;

        assert!(matches!(s, SensorType::AccelerometerSensor(_)));

        Ok(())
    }
}
