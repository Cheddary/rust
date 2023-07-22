use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct IpResponse {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // The URL to make the HTTP request to
    let url = "https://httpbin.org/ip";

    // Make the HTTP GET request
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Read the response body as JSON
        let json_response: Value = response.json().await?;

        // Deserialize the JSON into the IpResponse struct
        let ip_response: IpResponse = serde_json::from_value(json_response).unwrap();

        // Print the IP address
        println!("IP Address: {}", ip_response.origin);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}