
fn main() -> Result<(), reqwest::Error> {
    let token = dotenv::var("BEARER_TOKEN").unwrap();
    dotenv::dotenv().ok();
    println!("{}", token);

    let body = reqwest::blocking::get("http://example.com/")?.text()?;
    println!("body = {:?}", body);
    Ok(())
}
