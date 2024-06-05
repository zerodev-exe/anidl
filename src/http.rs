pub async fn get_html(url: String) -> String {
    let res = reqwest::get(url).await.unwrap();
    let body = res.text().await.unwrap();
    return body;
}
