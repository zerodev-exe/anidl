use reqwest;
use scraper::{Html, Selector};
use std::vec;

async fn get_html(url: String) -> String {
    let res = reqwest::get(url).await.unwrap();
    let body = res.text().await.unwrap();
    return body;
}

pub fn get_anime_episodes(url: &str) -> Vec<&str> {
    println!("{}", url);
    let urls = vec!["https://5af802wiv1. bd36019.com/user1342/2afb7116ad3734b467e49b206eb380ed/EP.100.v0.1639245168.720p.mp4?token=6cvIhK61KeUk5Aj5QsGsOw&expires=1715818860&id=107342"];
    urls
}

pub async fn parse(url: String) {
    let html = get_html(url);

    let fragment = Html::parse_document(html.await.as_str());
    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    if let Some(ul) = fragment.select(&ul_selector).next() {
        for element in ul.select(&li_selector) {
            assert_eq!("li", element.value().name());
        }
    }
}
