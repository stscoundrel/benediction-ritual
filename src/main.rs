use benediction_ritual::{get_headers, get_number_of_requests, get_request_payload};
use futures::future::join_all;
use reqwest::Client;

const API_URL: &str = "http://localhost:8080/api/events";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let num_requests = get_number_of_requests();
    let headers = get_headers();

    let request_futures = (0..num_requests).map(|_| {
        let payload = get_request_payload();

        client
            .post(API_URL)
            .headers(headers.clone())
            .body(payload.to_string())
            .send()
    });

    let responses = join_all(request_futures).await;

    for response in responses {
        let response = response?;
        println!("Response status: {:?}", response.status());
    }

    Ok(())
}
