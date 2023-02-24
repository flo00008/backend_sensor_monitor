use futures_util::StreamExt;
use warp::{ws::WebSocket, Filter};

use crate::{mongo::DbClient, sensors::SensorType};

pub fn get_sensor_routes(
    db: DbClient,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("lightsensor")
        .or(warp::path!("accelerometer"))
        .or(warp::path!("geolocation"))
        .or(warp::path!("orientation"))
        .or(warp::path!("proximity"))
        .and(warp::get()) // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .and(with_db(db))
        .map(|_, ws: warp::ws::Ws, db: DbClient| {
            ws.on_upgrade(move |websocket| handle_sensor(websocket, db))
        })
}

async fn handle_sensor(mut ws: WebSocket, db: DbClient) {
    while let Some(Ok(msg)) = ws.next().await {
        if !msg.is_text() {
            continue;
        }

        let json = msg.to_str().unwrap();
        let Ok(sensor):Result<SensorType, serde_json::Error> = serde_json::from_str(json) else { println!("Unable to get the type of request from {:?}", json); continue };

        db.insert_sensor_data(&sensor)
            .await
            .expect(format!("Error storing {sensor:?}").as_str());
    }
}

fn with_db(
    db: DbClient,
) -> impl Filter<Extract = (DbClient,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
