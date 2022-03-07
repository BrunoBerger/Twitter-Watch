//TODO: Only fetch tweets since last time
//TODO: Properly handle Error::EnvVar

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let token = dotenv::var("TW_BEARER_TOKEN")?;
    let user = dotenv::var("TW_USER")?;
    let url = format!("https://api.twitter.com/2/users/{user}/tweets?max_results=5");
    println!("Sending req to {}", url);

    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).bearer_auth(token).send()?;
    println!("Status: {}", resp.status());
    

    let body = resp.text()?;
    let message: serde_json::Value = serde_json::from_str(&body)?;
    
    let search_words = ["shop", "buy", "available", "print", "shirt"];
    let mut matching: Vec<u64> = Vec::new();

    for tweet in message["data"].as_array().unwrap() {
        for word in search_words {
            if tweet["text"].as_str().unwrap().to_lowercase().contains(&word.to_lowercase()) {
                matching.push(tweet["id"].as_str().unwrap().parse::<u64>()?);
            }
        }
    }
    println!("Matching tweets: {:?}", matching);

    if !matching.is_empty() {
        // Construct email with match info
        let sender = dotenv::var("TW_MAIL_SENDER")?;
        let reciever = dotenv::var("TW_MAIL_TO")?;
        let links: String = matching.iter()
            .map(|i| format!(" https://twitter.com/i/web/status/{} \n", i))
            .collect();
        let email = lettre::Message::builder()
            .from(["raspberry pi <", &sender, ">"].join("").parse().unwrap())
            .to(["<", &reciever, ">"].join("").parse().unwrap())
            .subject("Twitter-Watch alert")
            .body(format!("Found some of your search words in these tweets:\n{}", links))
            .unwrap();

        // Open a remote connection to gmail
        let sender_pw = dotenv::var("TW_MAIL_PW")?;
        let creds = lettre::transport::smtp::authentication::Credentials::new(sender, sender_pw);
        let mailer = lettre::SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match lettre::Transport::send(&mailer, &email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e)
        }
    } else {
        println!("No email sent");
    }

    Ok(())
}
