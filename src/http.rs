pub async fn get_html(url: String) -> String {
    let res = reqwest::get(url).await.unwrap();
    let body = res.text().await.unwrap();
    return body;
}

pub async fn handle_redirect(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut response = client.get(url).send().await?;

    if response.status() == reqwest::StatusCode::FOUND {
        if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
            let new_url = location.to_str().unwrap();
            return Ok(new_url.to_string());
        }
    }

    Ok(url.to_string())
}
