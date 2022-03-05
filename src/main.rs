//TODO: Only fetch tweets since last time
//TODO: Properly handle Error::EnvVar
//TODO: Properly parse the resp , maybe with https://github.com/twitter/twitter-text

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let token = dotenv::var("TW_BEARER_TOKEN")?;
    let user = dotenv::var("TW_USER")?;
    let url = format!("https://api.twitter.com/2/users/{user}/tweets?max_results=5");
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
    let mut contains_word = false;
    for word in search_words.iter() {
        if message.to_lowercase().contains(word) {
            contains_word = true;
        }
        println!("contains '{word}': {contains_word}", );
    }

    if contains_word {
        let sender = dotenv::var("TW_MAIL_SENDER")?;
        let reciever = dotenv::var("TW_MAIL_TO")?;
        let email = lettre::Message::builder()
            .from(["raspberry pi <", &sender, ">"].join("").parse().unwrap())
            .to(["<", &reciever,">"].join("").parse().unwrap())
            .subject("Twitter-Watch alert")
            .body(String::from("Found some of your search words"))
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
    }

    Ok(())
}
