extern crate csv;
extern crate rand;
extern crate egg_mode;
extern crate rustc_serialize;

use std::collections::{HashMap, HashSet};
use std::path::Path;

const CSV_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/pokedex/pokedex/data/csv");

struct Env {
    pub pokemon_names: HashMap<u32, String>,
    pub pokemon_moves: HashMap<u32, HashSet<u32>>,
    pub move_names: HashMap<u32, String>,
}

fn main() {
    let env = Env::load();
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let (ref id, ref name) = rand::sample(&mut rng, &env.pokemon_names, 1)[0];
        let moves = rand::sample(&mut rng, env.pokemon_moves.get(id).unwrap(), 4);

        print!("{} with ", name);

        for (idx, move_id) in moves.into_iter().enumerate() {
            if idx > 0 {
                print!(", ");
            }
            print!("{}", env.move_names.get(move_id).unwrap());
        }
        println!("");
    }
}

impl Env {
    fn load() -> Env {
        let csv_dir = Path::new(CSV_PATH);

        let mut name_rdr = csv::Reader::from_file(csv_dir.join("pokemon_species_names.csv"))
                                   .unwrap()
                                   .has_headers(true);
        let mut pokemon_names = HashMap::new();
        for s_row in name_rdr.decode::<SpeciesName>().filter_map(|n| n.ok()).filter(|n| n.lang_id == 9) {
            pokemon_names.insert(s_row.species_id, s_row.name);
        }

        let mut move_rdr = csv::Reader::from_file(csv_dir.join("pokemon_moves.csv"))
                                   .unwrap()
                                   .has_headers(true);
        let mut pokemon_moves = HashMap::new();
        for m_row in move_rdr.decode::<PokemonMove>().filter_map(|m| m.ok()).filter(|m| m.group_id == 16) {
            let moves = pokemon_moves.entry(m_row.pokemon_id).or_insert(HashSet::new());
            moves.insert(m_row.move_id);
        }

        let mut move_name_rdr = csv::Reader::from_file(csv_dir.join("move_names.csv"))
                                        .unwrap()
                                        .has_headers(true);
        let mut move_names = HashMap::new();
        for n_row in move_name_rdr.decode::<MoveName>().filter_map(|n| n.ok()).filter(|n| n.lang_id == 9) {
            move_names.insert(n_row.move_id, n_row.name);
        }

        Env {
            pokemon_names: pokemon_names,
            pokemon_moves: pokemon_moves,
            move_names: move_names,
        }
    }
}

#[derive(RustcDecodable)]
struct SpeciesName {
    species_id: u32,
    lang_id: u32,
    name: String,
    _genus: String,
}

#[derive(RustcDecodable)]
struct PokemonMove {
    pokemon_id: u32,
    group_id: u32,
    move_id: u32,
    _method_id: u32,
    _level: u32,
    _order: Option<u32>,
}

#[derive(RustcDecodable)]
struct MoveName {
    move_id: u32,
    lang_id: u32,
    name: String,
}
