mod api_request;
mod io_streams;
mod pokemon;
mod request_pokemon;

use async_std;
use json;

#[async_std::main]
async fn main() {
    let generation = io_streams::get_input("Enter generation for dataset: ");
    let body = json::parse(&request_pokemon::request_all_pokemons(&generation).await).unwrap();
    let mut filename = io_streams::get_input("Write filename output: ");
    filename += ".csv";
    println!("Clearing file: {}", filename);
    io_streams::clear_file(&filename);
    println!("Writing Dataset into file: {}", filename);
    io_streams::print_in_file(&filename, "id,pokedex,en_name,fr_name,hp,atk,def,spa,spd,spe,type_1,type_2,ability_1,ability_2,ability_hidden,tier,gen\n", false);
    let all_pokemons = pokemon::create_all_pokemons(&body["pokemons"]);
    pokemon::print_all_pokemons(&all_pokemons, &filename);
    println!("Dataset written into file: {}", filename);
}
