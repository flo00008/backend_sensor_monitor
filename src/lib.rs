use std::{env, net::SocketAddr};

use routes::get_sensor_routes;
use warp::Filter;

mod routes;
mod sensors;

pub async fn setup_server(addr: impl Into<SocketAddr>) {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "monitor::api=info");
    }

    pretty_env_logger::init();
    let log = warp::log("monitor::api");

    let track_sensor = get_sensor_routes();

    let routes = track_sensor.with(log);

    warp::serve(routes).run(addr).await;
}
