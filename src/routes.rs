use futures_util::StreamExt;
use warp::{ws::WebSocket, Filter};

use crate::sensors::SensorType;

use self::handlers::*;

pub fn get_sensor_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("lightsensor")
        .or(warp::path!("accelerometer"))
        .and(warp::get()) // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|_, ws: warp::ws::Ws| {
            // And then our closure will be called when it compl
            ws.on_upgrade(move |websocket| handle_sensor(websocket))
        })
}

async fn handle_sensor(mut ws: WebSocket) {
    while let Some(Ok(msg)) = ws.next().await {
        if !msg.is_text() {
            continue;
        }

        let json = msg.to_str().unwrap();
        let Ok(sensor):Result<SensorType, serde_json::Error> = serde_json::from_str(json) else { println!("Unable to get the type of request from {:?}", json); continue };

        match sensor {
            SensorType::LightSensor(sensor) => handle_lightsensor(sensor),
            SensorType::AccelerometerSensor(sensor) => handle_accelerometersensor(sensor),
        }
    }
}

mod handlers {
    use crate::sensors::*;

    pub fn handle_lightsensor(sensor_data: Light) {
        println!("{:?}", sensor_data);
    }

    pub fn handle_accelerometersensor(sensor_data: Accelerometer) {
        println!("{:?}", sensor_data);
    }
}
