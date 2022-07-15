use crate::io_streams;
use json::{self, JsonValue};

pub struct Type {
    pub id: u32,
    pub en_name: String,
    pub fr_name: String,
    pub gen: u32,
}

impl Type {
    fn new(id: u32, en_name: String, fr_name: String, gen: u32) -> Type {
        Type {
            id,
            en_name,
            fr_name,
            gen,
        }
    }
}

pub struct Ability {
    pub id: u32,
    pub fr_name: String,
    pub en_name: String,
    pub gen: u32,
}

impl Ability {
    fn new(id: u32, fr_name: String, en_name: String, gen: u32) -> Ability {
        Ability {
            id,
            fr_name,
            en_name,
            gen,
        }
    }
}

pub struct Pokemon {
    pub id: u16,
    pub en_name: String,
    pub fr_name: String,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub spa: u32,
    pub spd: u32,
    pub spe: u32,
    pub type_1: Type,
    pub type_2: Type,
    pub ability_1: Ability,
    pub ability_2: Ability,
    pub ability_hidden: Ability,
    pub tier: String,
    pub gen: u32,
}

impl Pokemon {
    fn new(id: u16, en_name: String, fr_name: String, hp: u32, atk: u32, def: u32, spa: u32, spd: u32, spe: u32, type_1: Type, type_2: Type, ability_1: Ability, ability_2: Ability, ability_hidden: Ability, tier: String, gen: u32) -> Pokemon {
        Pokemon {
            id,
            en_name,
            fr_name,
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
            type_1,
            type_2,
            ability_1,
            ability_2,
            ability_hidden,
            tier,
            gen,
        }
    }

    fn to_string(self: &Self) -> String {
        let mut string = String::new();
        string += &format!("{},", self.id);
        string += &format!("{},", self.en_name);
        string += &format!("{},", self.fr_name);
        string += &format!("{},", self.hp);
        string += &format!("{},", self.atk);
        string += &format!("{},", self.def);
        string += &format!("{},", self.spa);
        string += &format!("{},", self.spd);
        string += &format!("{},", self.spe);
        string += &format!("{},", self.type_1.en_name);
        string += &format!("{},", self.type_2.en_name);
        string += &format!("{},", self.ability_1.en_name);
        string += &format!("{},", self.ability_2.en_name);
        string += &format!("{},", self.ability_hidden.en_name);
        string += &format!("{},", self.tier);
        string += &format!("{}", self.gen);
        string
    }
}


pub fn create_all_pokemons(_pokemons: &JsonValue) -> Vec<Pokemon> {
    let mut all_pokemons = Vec::new();
    for pokemon in _pokemons.members() {
        all_pokemons.push(Pokemon::new(
        pokemon["pokedex"].as_u16().unwrap(),
    pokemon["name"].as_str().unwrap().to_string(),
    if pokemon["nom"].is_null() {"".to_string()} else { pokemon["nom"].as_str().unwrap().to_string()},
     pokemon["hp"].as_u32().unwrap(),
     pokemon["atk"].as_u32().unwrap(),
     pokemon["def"].as_u32().unwrap(),
     pokemon["spa"].as_u32().unwrap(),
     pokemon["spd"].as_u32().unwrap(),
        pokemon["spe"].as_u32().unwrap(),
        Type::new(pokemon["type_1"]["id"].as_u32().unwrap(), pokemon["type_1"]["name"].as_str().unwrap().to_string(), pokemon["type_1"]["nom"].as_str().unwrap().to_string(), pokemon["type_1"]["gen"].as_u32().unwrap()),
        if pokemon["type_2"].is_null() {
            Type::new(0, "".to_string(), "".to_string(), 0)
        } else {
            Type::new(pokemon["type_2"]["id"].as_u32().unwrap(), pokemon["type_2"]["name"].as_str().unwrap().to_string(), pokemon["type_2"]["nom"].as_str().unwrap().to_string(), pokemon["type_2"]["gen"].as_u32().unwrap())
        },
    if pokemon["ability_1"].is_null() {
        Ability::new(0, "".to_string(), "".to_string(), 0)
        } else {
            Ability::new(pokemon["ability_1"]["id"].as_u32().unwrap(), pokemon["ability_1"]["name"].as_str().unwrap().to_string(), pokemon["ability_1"]["nom"].as_str().unwrap().to_string(), pokemon["ability_1"]["gen"].as_u32().unwrap())
        },
         if pokemon["ability_2"].is_null() {
            Ability::new(0, "".to_string(), "".to_string(), 0)
        } else {
            Ability::new(pokemon["ability_2"]["id"].as_u32().unwrap(), pokemon["ability_2"]["nom"].as_str().unwrap().to_string(), pokemon["ability_2"]["name"].as_str().unwrap().to_string(), pokemon["ability_2"]["gen"].as_u32().unwrap())
        },
        if pokemon["ability_hidden"].is_null() {
            Ability::new(0, "".to_string(), "".to_string(), 0)
        } else {
            Ability::new(pokemon["ability_hidden"]["id"].as_u32().unwrap(), pokemon["ability_hidden"]["nom"].as_str().unwrap().to_string(), pokemon["ability_hidden"]["name"].as_str().unwrap().to_string(), pokemon["ability_hidden"]["gen"].as_u32().unwrap())
        },
        pokemon["tier"]["shortName"].as_str().unwrap().to_string(),
        pokemon["gen"].as_u32().unwrap()
        ));
    }
    all_pokemons
}

pub fn print_all_pokemons(all_pokemons: &Vec<Pokemon>, filename: &str) {
    let mut i = 1;
    for pokemon in all_pokemons {
        io_streams::print_in_file(filename, (i.to_string() + "," + pokemon.to_string().as_str() + "\n").as_str(), true); 
        i += 1;
    }
}