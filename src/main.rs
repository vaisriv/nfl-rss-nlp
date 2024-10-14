mod rss_read;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://rss.nytimes.com/services/xml/rss/nyt/ProFootball.xml",
        "https://api.foxsports.com/v2/content/optimized-rss?partnerKey=MB0Wehpmuj2lUhuRhQaafhBjAJqaPU244mlTDK1i&size=30&tags=fs/nfl",
        "https://www.espn.com/espn/rss/nfl/news",
        "https://www.cbssports.com/rss/headlines/nfl/"
    ];

    match rss_read::update_feeds(urls).await {
        Ok(df) => println!("{:?}", df),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
