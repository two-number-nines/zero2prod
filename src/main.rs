use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_config().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener =
        TcpListener::bind(address).expect("Should bind to socket address with port from config");
    run(listener)?.await
}
