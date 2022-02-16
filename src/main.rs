
fn blocking_get(url: &str, token: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url)
        .bearer_auth(token)
        .send()?;
    Ok(resp)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let token = dotenv::var("TW_BEARER_TOKEN").unwrap();
    let user = dotenv::var("TW_USER").unwrap();
    let word = dotenv::var("TW_WORD").unwrap();
    dotenv::dotenv().ok();
    let url = format!("https://api.twitter.com/2/users/{user}/tweets?max_results=5");
    println!("Sending req to {}", url);

    let resp = blocking_get(&url, &token)?;
    println!("Status: {}", resp.status());

    let message = resp.text()?;
    let split = message.split("{");
    for s in split {
        println!("{}", s)
    }

    println!("contains '{word}': {}", message.to_lowercase().contains(&word));
    Ok(())
}
