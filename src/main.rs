
fn blocking_get(url: &str, token: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let mut map = std::collections::HashMap::new();
    map.insert("lang", "rust");
    let resp = client.get(url)
        .bearer_auth(token)
        .json(&map)
        .header(reqwest::header::USER_AGENT, "what dis")
        .body("I am Boddy!")
        .send()?;
    Ok(resp)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let token = dotenv::var("BEARER_TOKEN").unwrap();
    let url = dotenv::var("URL").unwrap();
    dotenv::dotenv().ok();
    println!("Sending req to {}", url);

    let resp = blocking_get(&url, &token)?;
    println!("Status: {}", resp.status());
    println!("{:#?}", resp);
    println!("{:?}", resp.text());
    Ok(())
}
