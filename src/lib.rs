use std::{env, net::SocketAddr};

use mongo::DbClient;
use routes::get_sensor_routes;
use warp::{log::Log, Filter};

mod mongo;
mod routes;
mod sensors;

pub async fn setup_server(addr: impl Into<SocketAddr>) {
    let log = setup_log();
    let db = setup_db().await;

    let track_sensor = get_sensor_routes(db);

    let routes = track_sensor.with(log);

    warp::serve(routes).run(addr).await;
}

async fn setup_db() -> DbClient {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    DbClient::new(client_uri)
        .await
        .expect("Unable to get mongo client")
}

fn setup_log() -> Log<impl Fn(warp::log::Info) + Copy> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "monitor::api=info");
    }

    pretty_env_logger::init();
    warp::log("monitor::api")
}
