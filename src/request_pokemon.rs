use crate::api_request::get_request;

pub async fn request_all_pokemons(generation: &str) -> String {
    let base = String::from("https://www.coupcritique.fr/api/pokemons?gen=");
    let url = String::from(base + generation);
    let body = get_request(&url).await;
    body
}