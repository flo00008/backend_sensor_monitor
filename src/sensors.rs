use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensorType {
    LightSensor(Light),
    AccelerometerSensor(Accelerometer),
    GeolocationSensor(Geolocation),
    OrientationSensor(Orientation),
    ProximitySensor(Proximity),
}
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
pub struct Geolocation {
    payload: String,
    position: GeolocationPosition,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeolocationPosition {
    timestamp: u64,
    provider: String,
    mocked: bool,
    coords: GeolocationPositionCoords,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeolocationPositionCoords {
    accuracy: f32,
    altitude: f32,
    #[serde(rename = "altitudeAccuracy")]
    altitude_accuracy: f32,
    heading: f32,
    latitude: f32,
    longitude: f32,
    speed: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orientation {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    azimuth: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pitch: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    roll: f32,

    #[serde(flatten)]
    base: SensorBase,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Proximity {
    #[serde(rename = "IsNear")]
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    is_near: bool,
    #[serde(rename = "MaxRange")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    max_range: f32,
    #[serde(rename = "Value")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    value: f32,

    #[serde(flatten)]
    base: SensorBase,
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

    #[test]
    fn deserialize_geolocation_sensor() -> Result<(), serde_json::Error> {
        let json = "{\"position\":{\"timestamp\":1677259207375,\"mocked\":false,\"provider\":\"fused\",\"coords\":{\"speed\":0,\"heading\":0,\"altitude\":525.5,\"accuracy\":16.850000381469727,\"longitude\":-3.7842402,\"altitudeAccuracy\":2.4427359104156494,\"latitude\":37.7816604}},\"payload\":\"Movil\"}";

        let _: Geolocation = serde_json::from_str(json)?;

        let s: SensorType = serde_json::from_str(json)?;

        assert!(matches!(s, SensorType::GeolocationSensor(_)));

        Ok(())
    }

    #[test]
    fn deserialize_orientation_sensor() -> Result<(), serde_json::Error> {
        let json = "{\"SensorName\":\"Orientation\",\"Timestamp\":1677259355607,\"azimuth\":\"189.4746856689453\",\"pitch\":\"336.5517883300781\",\"roll\":\"359.8556823730469\",\"payload\":\"Movil\"}";

        let _: Orientation = serde_json::from_str(json)?;

        let s: SensorType = serde_json::from_str(json)?;

        assert!(matches!(s, SensorType::OrientationSensor(_)));

        Ok(())
    }

    #[test]
    fn deserialize_proximity_sensor() -> Result<(), serde_json::Error> {
        let json = "{\"SensorName\":\"Proximity\",\"Timestamp\":1677259406733,\"IsNear\":\"true\",\"Value\":\"0\",\"MaxRange\":\"5\",\"payload\":\"Movil\"}";

        let _: Proximity = serde_json::from_str(json)?;

        let s: SensorType = serde_json::from_str(json)?;

        assert!(matches!(s, SensorType::ProximitySensor(_)));

        Ok(())
    }
}
