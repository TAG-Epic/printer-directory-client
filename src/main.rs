use bore_cli::client::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let session = Client::new("localhost", 8000, "bore.pub", 0, None).await.unwrap();
    let port = session.remote_port();
    
    let http_client = reqwest::Client::new();

    let args = std::env::args().collect::<Vec<String>>();
    let printer_name = &args[1];
    let printer_type = &args[2];

    let body = json!({
        "name": printer_name,
        "type": printer_type,
        "url": format!("http://bore.pub:{port}")
    });
    let response = http_client.post(env!("SERVER_URL")).json(&body).send().await.expect("Failed to send request");
    response.error_for_status().unwrap();

    println!("Listening on {port}");
    session.listen().await.unwrap()
}
