use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // The URL to make the HTTP request to
    let url = "https://httpbin.org/ip";

    // Make the HTTP GET request
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Read the response body as a string
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}