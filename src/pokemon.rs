// Exemple of one pokemon recived:
// {
//     "id": 3108,
//     "pokedex": 1,
//     "name": "Bulbasaur",
//     "nom": "Bulbizarre",
//     "hp": 45,
//     "atk": 49,
//     "def": 49,
//     "spa": 65,
//     "spd": 65,
//     "spe": 45,
//     "type_1": {
//         "id": 90,
//         "name": "Grass",
//         "nom": "Plante",
//         "gen": 8
//     },
//     "type_2": {
//         "id": 129,
//         "name": "Poison",
//         "nom": "Poison",
//         "gen": 8
//     },
//     "ability_1": {
//         "id": 834,
//         "name": "Overgrow",
//         "nom": "Engrais",
//         "gen": 8
//     },
//     "ability_2": null,
//     "ability_hidden": {
//         "id": 362,
//         "name": "Chlorophyll",
//         "nom": "Chlorophylle",
//         "gen": 8
//     },
//     "tier": {
//         "id": 74,
//         "name": "Little Cup",
//         "shortName": "LC",
//         "playable": true,
//         "rank": 100,
//         "sortOrder": 100,
//         "isDouble": null,
//         "gen": 8
//     },
//     "technically": false,
//     "gen": 8
// }

struct Type {
    id: u32,
    name: String,
    nom: String,
    gen: u32,
}

struct Ability {
    id: u32,
    fr_name: String,
    en_name: String,
    gen: u32,
}

struct Pokemon {
    id: u16,
    name: String,
    hp: u32,
    atk: u32,
    def: u32,
    spa: u32,
    spd: u32,
    spe: u32,
    type_1: Type,
    type_2: Type,
    ability_1: Ability,
    ability_2: Ability,
    ability_hidden: Ability,
    tier: String,
    gen: u32,


}