use backend_sensor_monitor::setup_server;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    setup_server(([192, 168, 0, 13], 3030)).await;
}
