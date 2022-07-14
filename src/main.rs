mod api_request;
mod io_streams;
mod pokemon;
mod request_pokemon;

use async_std;

#[async_std::main]
async fn main() {
    let generation = io_streams::get_input("Enter generation for dataset: ");
    let body = request_pokemon::request_all_pokemons(&generation).await;
    let mut filename = io_streams::get_input("Write filename output: ");
    filename += ".csv";
    println!("Write Detaset in file: {}", filename);
}
