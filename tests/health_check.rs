use reqwest::Client;
use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let socket_address = spawn_app();

    let client = Client::new();

    let res = client.get(&format!("{}/health_check", socket_address)).send().await.expect("Should got a health check response");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Should bind to socket address with random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Server running with socket failed");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}