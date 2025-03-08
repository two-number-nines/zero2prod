use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_config().expect("Should get a configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Should connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener =
        TcpListener::bind(address).expect("Should bind to socket address with port from config");
    run(listener, connection_pool)?.await
}
