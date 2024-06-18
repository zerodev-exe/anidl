use colored::*;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

pub fn get_anime_url(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];
    let mut number_list = 0;

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        number_list += 1;
        let num = format!("[{}]", number_list);
        println!("{} - {}", num.red(), node.text());
        let temp_val = node.attr("href").unwrap().split("/");
        let anime_url_ending = temp_val.last().expect("Someting went wrong");
        anime_list.push(anime_url_ending.to_string())
    }

    anime_list
}

pub fn get_anime_name(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "name").descendant(Name("a"))) {
        anime_list.push(node.text());
    }

    anime_list
}

pub fn get_video_url(body: String) -> Vec<String> {
    let mut anime_list: Vec<_> = vec![];

    let document = Document::from(body.as_str());
    for node in document.find(Attr("class", "cf-download").descendant(Name("a"))) {
        anime_list.push(node.attr("href").unwrap().to_string());
    }

    anime_list
}

// TODO: Actaully use the the urls in the test cases
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_anime_url() {
        let body =
            r#"<div class="name"><a href="/anime/one-piece">One Piece</a></div>"#.to_string();

        let urls = get_anime_url(body);
        assert_eq!(urls, vec!["one-piece"]);
    }

    #[tokio::test]
    async fn test_get_anime_name() {
        let body = r#"<div class="name"><a>One Piece</a></div>"#.to_string();

        let names = get_anime_name(body);
        assert_eq!(names, vec!["One Piece"]);
    }

    #[tokio::test]
    async fn test_get_video_url() {
        let body =
            r#"<div class="cf-download"><a href="http://example.com/video.mp4">Download</a></div>"#
                .to_string();

        let urls = get_video_url(body);
        assert_eq!(urls, vec!["http://example.com/video.mp4"]);
    }
}
