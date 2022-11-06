use std::net::TcpListener;

#[tokio::test]
async fn health_check_succeeds() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request to /health_check");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = silverbook::run(listener).await.expect("Failed to listen");

    tokio::spawn(server);

    return format!("http:127.0.0.1:{}", port);
}
