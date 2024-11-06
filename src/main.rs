
use clap::Parser;
use reqwest::{header::HeaderMap, Response};

#[tokio::main]
async fn get_request(url: String) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    
    Ok(response)
}

fn assess_headers(response_headers: &HeaderMap) {
    if response_headers.get("x-frame-options").unwrap() == &"SAMEORIGIN" {
        println!("SAMEORIGIN set for X-FRAME-OPTIONS header, this header is deprecated and can be kept for compatibility purposes for old browsers but should not be relied upon.\nUse 'frame-ancestors' and 'content-security-policy' headers instead. See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options.")
    } else if response_headers.get("x-frame-options").is_none() {
        println!("No X-FRAME-OPTIONS header found, check for 'frame-ancestors' and 'content-security-policy' headers.")
    }
}

#[derive(Parser)]
struct Args {
    url: String
}

fn main() {
    let args = Args::parse();
    // let headers = get_request(args.url).unwrap();
    match get_request(args.url) {
        Ok(response) => {
            let response_headers = response.headers();
            println!("Headers:\n{:#?}", response_headers);
            assess_headers(response_headers);
        },
        Err(e) => {
            println!("Error calling url: {}", e)
        }
    }

}