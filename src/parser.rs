use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name, Predicate};

/// Extracts the URLs of anime from the provided HTML body.
/// This function searches for 'a' tags within elements with class 'name'.
///
/// # Arguments
/// * `body` - HTML content as a string.
///
/// # Returns
/// A `Vec<String>` containing the URL endings of each anime.
pub fn get_anime_url(body: String) -> Vec<String> {
    let document = parse_document(&body);
    let nodes = find_nodes(&document, "name");

    let anime_list: Vec<String> = nodes
        .iter()
        .filter_map(|node| {
            node.attr("href")
                .and_then(|href| href.split('/').last())
                .map(|url_ending| url_ending.to_string())
        })
        .collect();

    anime_list
}

/// Extracts the names of anime from the provided HTML body.
///
/// # Arguments
///
/// * `body` - A string containing the HTML content.
///
/// # Returns
///
/// A vector of strings, each representing the name of an anime.
pub fn get_anime_name(body: String) -> Vec<String> {
    let document = parse_document(&body);
    let nodes = find_nodes(&document, "name");

    nodes.iter().map(|node| node.text()).collect()
}

/// Extracts the video URLs from the provided HTML body.
///
/// # Arguments
///
/// * `body` - A string containing the HTML content.
///
/// # Returns
///
/// A vector of strings, each representing a video URL.
pub fn get_media_url(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];

    let document = parse_document(&body);
    for node in document.find(Attr("class", "cf-download").descendant(Name("a"))) {
        anime_list.push(node.attr("href").unwrap().to_string());
    }

    anime_list
}

pub fn get_anime_images(body: String) -> Vec<String> {
    let mut anime_list: Vec<String> = vec![];

    let document = parse_document(&body);
    for node in document.find(Attr("class", "img").descendant(Name("img"))) {
        if let Some(src) = node.attr("src") {
            anime_list.push(src.to_string());
        }
    }

    anime_list
}

pub fn get_total_number_of_episodes(body: String) -> Result<u32, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(&body);
    let episode_selector = scraper::Selector::parse("div.anime_video_body>ul li a").unwrap();

    let last_episde = document.select(&episode_selector);

    let first_episode = last_episde.last(); // Get the first item

    if let Some(episode) = first_episode {
        return Ok(episode
            .inner_html()
            .split("-")
            .last()
            .expect("An error has ocured")
            .parse::<u32>()
            .unwrap());
    }

    Err("An error has occurred".into())
}

fn parse_document(body: &str) -> Document {
    Document::from(body)
}

fn find_nodes<'a>(document: &'a Document, selector: &'a str) -> Vec<Node<'a>> {
    document
        .find(Attr("class", selector).descendant(Name("a")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utils, CAT_URL, URL};
    use tokio;

    /// Tests the `get_anime_url` function.
    #[tokio::test]
    async fn test_get_anime_url() {
        let body = utils::get_html(format!("{}{}", URL, "search.html?keyword=Kenka%20Dokugaku"))
            .await
            .unwrap();

        let urls = get_anime_url(body);
        assert_eq!(
            urls,
            vec![
                "kenka-dokugaku".to_string(),
                "kenka-dokugaku-dub".to_string()
            ]
        );
    }

    /// Tests the `get_anime_name` function.
    #[tokio::test]
    async fn test_get_anime_name() {
        let body = utils::get_html(format!("{}{}", URL, "search.html?keyword=kenka%20dokugaku"))
            .await
            .unwrap();

        let names = get_anime_name(body);
        println!("{:?}", names);
        assert_eq!(names, vec!["Kenka Dokugaku", "Kenka Dokugaku (Dub)"]);
    }

    #[tokio::test]
    async fn test_get_anime_url_with_empty_body() {
        let body = "".to_string();
        let urls = get_anime_url(body);
        assert!(urls.is_empty());
    }

    #[tokio::test]
    async fn test_get_total_number_of_episodes_kaijuu8() {
        let body = utils::get_html(format!("{CAT_URL}kaijuu-8-gou-dub"))
            .await
            .unwrap();
        assert_eq!(get_total_number_of_episodes(body).unwrap(), 12)
    }
    #[tokio::test]
    async fn test_get_total_number_of_episodes_one_piece() {
        let body = utils::get_html(format!("{CAT_URL}one-piece"))
            .await
            .unwrap();
        assert_eq!(get_total_number_of_episodes(body).unwrap(), 1119)
    }

    #[tokio::test]
    async fn test_get_total_number_of_episodes_bleach() {
        let body = utils::get_html(format!("{CAT_URL}bleach")).await.unwrap();
        assert_eq!(get_total_number_of_episodes(body).unwrap(), 366)
    }
}
