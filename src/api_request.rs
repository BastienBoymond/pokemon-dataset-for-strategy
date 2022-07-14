use reqwest::{get};

pub async fn get_request(url: &str) -> String {
    let body = get(url).await.unwrap().text().await.unwrap();
    body
}