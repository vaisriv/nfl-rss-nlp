mod rss_read;

#[tokio::main]
async fn main() {
    let urls = vec!["https://rss.nytimes.com/services/xml/rss/nyt/ProFootball.xml"];

    match rss_read::update_feeds(urls).await {
        Ok(df) => println!("{:?}", df),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
