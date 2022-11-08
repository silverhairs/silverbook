#[tokio::test]
async fn subscribe_should_return_200_status_code_when_form_data_is_valid() {
    let client = reqwest::Client::new();
    let address = test_helpers::spawn_app().await;
    let payload = "name=john%20doe&email=john_doe%40gmail.com";

    let res = client
        .post(&format!("{}/subscribe", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(payload)
        .send()
        .await
        .expect("Failed to make request");

    assert!(res.status().is_success());
}

#[tokio::test]
async fn subscribe_should_return_400_status_when_form_data_is_invalid() {
    let client = reqwest::Client::new();
    let address = test_helpers::spawn_app().await;

    let test_cases = vec![
        ("name=john%20doe", "missing email address"),
        ("email=john_doe%40gmail.com", "missing name"),
        ("", "missing name and email address"),
    ];

    for (body, error) in test_cases {
        let res = client
            .post(&format!("{}/subscribe", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(&*body)
            .send()
            .await
            .expect(&format!("Failed to make request with body {}", body));

        assert_eq!(
            res.status().as_u16(),
            400,
            "expected a 400 status with error {}",
            error
        );
    }
}

mod test_helpers {
    use std::net::TcpListener;

    pub async fn spawn_app() -> String {
        let lst = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
        let port = lst.local_addr().unwrap().port();
        let server = silverbook::run(lst)
            .await
            .expect("Failed to spin off server");

        tokio::spawn(server);

        return format!("http://127.0.0.1:{}", port);
    }
}
