use reqwest::Client;
use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let socket_address = spawn_app();
    let client = Client::new();
    let res = client.get(&format!("{}/health_check", &socket_address)).send().await.expect("Should got a health check response");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let socket_address = spawn_app();
    let client = Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let res = client
        .post(&format!("{}/subscriptions", &socket_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Should got a valid subscription response");

    assert_eq!(200, res.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let socket_address = spawn_app();
    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client.post(&format!("{}/subscriptions", &socket_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await.expect("Should got an invalid subscription response");

        assert_eq!(response.status().as_u16(), 400, "The API didn't fail with a 400 when the payload was {}", error_message);
    }



}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Should bind to socket address with random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Server running with socket failed");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}