use crate::rss_read::*;
use polars::prelude::*;

async fn get_rss(verbose: bool) -> Result<Series, Box<dyn std::error::Error>> {
    let urls = vec![
        "https://rss.nytimes.com/services/xml/rss/nyt/ProFootball.xml",
        "https://api.foxsports.com/v2/content/optimized-rss?partnerKey=MB0Wehpmuj2lUhuRhQaafhBjAJqaPU244mlTDK1i&size=30&tags=fs/nfl",
        "https://www.espn.com/espn/rss/nfl/news",
        "https://www.cbssports.com/rss/headlines/nfl/",
        "https://profootballmania.com/feed/",
        "https://feeds.washingtonpost.com/rss/rss_football-insider",
        "https://sportspyder.com/nfl/philadelphia-eagles/news.xml",
        "https://www.thecoldwire.com/sports/nfl/philadelphia-eagles/feed/",
    ];

    let rss_feeds = update_feeds(urls).await;

    if verbose {
        match rss_feeds {
            Ok(ref rss_feeds) => println!("{:?}", rss_feeds),
            Err(ref e) => eprintln!("Error: {:?}", e),
        };
    }

    let rss_df = rss_feeds.unwrap();
    let rss_col = extract_col(&rss_df, "description").await;

    if verbose {
        match rss_col {
            Ok(ref rss_col) => println!("{:?}", rss_col),
            Err(ref e) => eprintln!("Error: {:?}", e),
        };
    }

    rss_col
}

pub async fn get_context_str(verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let context_series = get_rss(verbose).await?;
    let context_str = std::string::ToString::to_string(&context_series);

    // let context_str = String::from("The next Super Bowl will be won by the Kansas City Chiefs");
    Ok(context_str)
}
