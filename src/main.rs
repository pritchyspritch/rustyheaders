
use clap::Parser;
use reqwest::{header::HeaderMap, Response};
use serde::Deserialize;
use std::collections::HashMap;

async fn get_request(url: String) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    
    Ok(response)
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct SecurityHeader {
    name: String,
    header_value: String,
    description: String,
    misconfigured: bool,
    link: String
}

impl SecurityHeader {
    fn new(name: &str, header_value: &str, description: &str, misconfigured: bool, link: &str) -> SecurityHeader {
        SecurityHeader {
            name: name.to_string(), header_value: header_value.to_string(), description: description.to_string(), misconfigured, link: link.to_string()
        }
    }
}
#[derive(Deserialize)]
struct HeaderDefinition {
    description: String,
    misconfigured: bool,
    link: String
}

fn header_hashy_from_file() -> HashMap<String, HashMap<String, HeaderDefinition>> {
    serde_json::from_str(include_str!("config.json")).unwrap()
}

fn header_hashy() -> HashMap<String, HashMap<String, HeaderDefinition>> {
    let mut header_definition = HashMap::new();
    let mut header_hashy = HashMap::new();
    header_definition.insert("SAMEORIGIN".to_string(), HeaderDefinition { description: "description".to_string(), misconfigured: true, link: "link".to_string() });
    header_hashy.insert("x-frame-options".to_string(), header_definition);
    header_hashy
}

fn assess_headers(response_headers: &HeaderMap) {

    let hh = header_hashy_from_file();

    let mut security_findings = vec![];

    for (k,v) in response_headers {
        let (k,v) = (k.as_str(), v.to_str().unwrap());
        let header_definition = hh.get(k).and_then(|header| header.get(v));
        if let Some(hd) = header_definition {
            let sh = SecurityHeader::new(k, v, &hd.description, hd.misconfigured, &hd.link);
            security_findings.push(sh);
        }
    }

    dbg!(security_findings);

    let mut security_headers: HashMap<&str, SecurityHeader> = HashMap::new();

    if response_headers.get("x-frame-options").map(|v| v == "SAMEORIGIN") == Some(true) {
        let name = "x-frame-options";
        let header_value = "SAMEORIGIN";
        let description = "SAMEORIGIN set for X-FRAME-OPTIONS header, this header is deprecated and can be kept for compatibility purposes for old browsers but should not be relied upon. Use 'frame-ancestors' and 'content-security-policy' headers instead.";
        let misconfigured = true;
        let link = "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options";
        
        security_headers.insert("xframeoptions", SecurityHeader::new(name, header_value, description, misconfigured, link));
        
        // println!("SAMEORIGIN set for X-FRAME-OPTIONS header, this header is deprecated and can be kept for compatibility purposes for old browsers but should not be relied upon.\nUse 'frame-ancestors' and 'content-security-policy' headers instead. See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options.")
    
    } else if response_headers.get("x-frame-options").is_none() {
        println!("No X-FRAME-OPTIONS header found, check for 'frame-ancestors' and 'content-security-policy' headers.")
    } else {
        println!("Other: {response_header_value:?}", response_header_value = response_headers.get("x-frame-options") )
    }

    for (key, value) in security_headers {
        println!("key: {key:?}\nname: {name:?}\nvalue: {header_value:?}\ndescription: {description:?}\nmisconfigured: {misconfigured:?}\nlink: {link:?}",
        name = value.name, 
        header_value = value.header_value,
        description = value.description,
        misconfigured = value.misconfigured,
        link = value.link,
        );
    }
}

#[derive(Parser)]
struct Args {
    url: String
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // let headers = get_request(args.url).unwrap();
    match get_request(args.url).await {
        Ok(response) => {
            let response_headers = response.headers();
            // println!("Headers:\n{:#?}", response_headers);
            assess_headers(response_headers);
        },
        Err(e) => {
            println!("Error calling url: {}", e)
        }
    }

}