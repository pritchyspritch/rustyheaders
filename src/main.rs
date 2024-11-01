
use clap::Parser;
use reqwest::Response;

#[tokio::main]
async fn get_request(url: String) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let headers = client
        .get(url)
        .send()
        .await?;
    
    Ok(headers)
}

#[derive(Parser)]
struct Args {
    url: String
}

fn main() {
    let args = Args::parse();
    // let headers = get_request(args.url).unwrap();
    match get_request(args.url) {
        Ok(headers) => {
            println!("Headers:\n{:#?}", headers.headers());
        },
        Err(e) => {
            println!("Error calling url: {}", e)
        }
    }

}