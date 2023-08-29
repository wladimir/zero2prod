//! tests/health_check.rs
use std::net::TcpListener;
use zero2prod::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to spawn");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to send");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_json_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "{\"name\":\"my-name\", \"email\": \"a@b.com\"}";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "{}";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(400, response.status().as_u16(), "API should return 400")
}
