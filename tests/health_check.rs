use std::net::TcpListener;

const LOCAL_IP: &str = "127.0.0.1";
const ANY_PORT: i32 = 0;

fn spawn_app() -> String {
    let any_addr = format!("{}:{}", LOCAL_IP, ANY_PORT);
    let listener = TcpListener::bind(any_addr).expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://{}:{}", LOCAL_IP, port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
