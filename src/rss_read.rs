use polars::prelude::*;
use reqwest::Client;
use rss::Channel;
use tokio::task;

pub async fn update_feeds(urls: Vec<&str>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut lazy_frames = Vec::new();

    for url in urls {
        let rss_feed = fetch_rss_feed(url, &client).await?;
        lazy_frames.push(rss_feed.lazy());
    }

    let concatenated_lazyframe = concat(lazy_frames.clone(), UnionArgs::default())?;

    let all_feeds_df = task::spawn_blocking(move || concatenated_lazyframe.collect()).await??;

    Ok(all_feeds_df)
}

async fn fetch_rss_feed(
    url: &str,
    client: &Client,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?.bytes().await?;
    let channel = Channel::read_from(&response[..])?;

    let titles: Vec<String> = channel
        .items()
        .iter()
        .map(|item| item.title().unwrap_or_default().to_string())
        .collect();

    let links: Vec<String> = channel
        .items()
        .iter()
        .map(|item| item.link().unwrap_or_default().to_string())
        .collect();

    let descriptions: Vec<String> = channel
        .items()
        .iter()
        .map(|item| item.description().unwrap_or_default().to_string())
        .collect();

    let pub_dates: Vec<String> = channel
        .items()
        .iter()
        .map(|item| item.pub_date().unwrap_or_default().to_string())
        .collect();

    let rss_posts = df![
        "title" => titles,
        "link" => links,
        "description" => descriptions.clone(),
        "pub_date" => pub_dates.clone(),
        "date_description" => pub_dates.iter().zip(descriptions.iter()).map(|(ref p, ref d)| p.to_owned().to_owned()+": "+d.to_owned()).collect::<Vec<String>>(),
    ]?;

    Ok(rss_posts)
}

pub async fn extract_col(
    rss_posts: &DataFrame,
    column: &str
) -> Result<Series, Box<dyn std::error::Error>> {
    let descs_df: Series = rss_posts[column].clone();

    Ok(descs_df)
}
