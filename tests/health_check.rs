use reqwest::Client;
use sqlx::{query, Executor, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::{get_config, DatabaseSettings};
use zero2prod::startup::run;

pub struct TestApp {
    socket_addr: String,
    connection_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = Client::new();
    let res = client
        .get(&format!("{}/health_check", &app.socket_addr))
        .send()
        .await
        .expect("Should got a health check response");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let res = client
        .post(&format!("{}/subscriptions", &app.socket_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Should got a valid subscription response");

    assert_eq!(200, res.status().as_u16());

    let saved = query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.connection_pool)
        .await
        .expect("Should get a record from subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.socket_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Should got an invalid subscription response");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API didn't fail with a 400 when the payload was {}",
            error_message
        );
    }
}

async fn spawn_app() -> TestApp {
    let mut configuration = get_config().expect("Should get a configuration");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Should bind to socket address with random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, connection_pool.clone()).expect("Server running with socket failed");
    let _ = tokio::spawn(server);
    let socket_addr = format!("http://127.0.0.1:{}", port);

    TestApp {
        socket_addr,
        connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection_pool = PgPool::connect(&config.connection_string_without_db())
        .await
        .expect("Should connect to Postgres");

    connection_pool
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name,).as_str())
        .await
        .expect("Should execute create database query");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Should connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Should run migrations");

    connection_pool
}
