
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let token = dotenv::var("TW_BEARER_TOKEN")?;
    let user = dotenv::var("TW_USER")?;
    let url = format!("https://api.twitter.com/2/users/{user}/tweets?max_results=10");
    println!("Sending req to {}", url);

    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).bearer_auth(token).send()?;

    println!("Status: {}", resp.status());
    let message = resp.text()?;
    // println!("{message}");
    // for s in message.split(&['{', '}']) {
    //     println!("{s}")
    // }
    let search_words = ["shop", "buy", "available", "print"];
    for word in search_words.iter() {
        println!("contains '{word}': {}", message.to_lowercase().contains(word));
    }
    
    Ok(())
}
