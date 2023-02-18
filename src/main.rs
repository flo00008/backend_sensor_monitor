use futures_util::StreamExt;
use std::env;
use warp::{ws::WebSocket, Filter};

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "monitor::api=info");
    }

    pretty_env_logger::init();
    let log = warp::log("monitor::api");

    let track_sensor = warp::get()
        .and(
            warp::path!("accelerometer").or(warp::path!("lightsensor")), // .or(warp::path!("gyroscope")),
        )
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|_path, ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(move |websocket| handle_request(websocket))
        });

    let routes = track_sensor.with(log);

    warp::serve(routes).run(([192, 168, 0, 13], 3030)).await;
}

async fn handle_request(mut ws: WebSocket) {
    while let Some(Ok(msg)) = ws.next().await {
        // TODO: Deserialize msg text
        println!("{:?}", msg);
    }
}
