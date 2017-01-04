extern crate csv;
extern crate rand;
extern crate egg_mode;
extern crate rustc_serialize;

use std::collections::{HashMap, HashSet};
use std::path::Path;

mod twitter;

const CSV_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/pokedex/pokedex/data/csv");

fn main() {
    let mut skip_twitter = false;
    let mut print_help = false;

    for arg in std::env::args() {
        if arg == "-h" || arg == "--help" {
            print_help = true;
        }
        if arg == "-s" || arg == "--skip-twitter" {
            skip_twitter = true;
        }
    }

    if print_help {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        println!("Generates random Pokemon movesets and item holding.");
        println!("Usage:");
        println!("    -h, --help: Print this message.");
        println!("    -s, --skip-twitter: Instead of logging into Twitter and posting there,");
        println!("                        prints one moveset to standard out.");
        return;
    }

    println!("loading pokemon information...");
    let env = Env::load();

    if !skip_twitter {
        let config = twitter::Config::load();

        let mon = make_pokemon(&env).unwrap();
        println!("new post: {}", mon);
        let post = egg_mode::tweet::DraftTweet::new(&mon);
        post.send(&config.con_token, &config.access_token).unwrap();
    }
    else {
        println!("{}", make_pokemon(&env).unwrap());
    }
}

fn make_pokemon(env: &Env) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut ret = String::new();
    let mut rng = rand::thread_rng();

    let (ref id, ref name) = rand::sample(&mut rng, &env.pokemon_names, 1)[0];
    let moves = rand::sample(&mut rng, env.pokemon_moves.get(id).unwrap(), 4);

    let item = rand::sample(&mut rng, &env.items, 1)[0];

    writeln!(ret, "{} @ {}", name, item)?;

    let ability_id = rand::sample(&mut rng, env.pokemon_abilities.get(id).unwrap(), 1)[0];
    writeln!(ret, "{}", env.ability_names.get(ability_id).unwrap())?;

    for move_id in moves {
        writeln!(ret, "- {}", env.move_names.get(move_id).unwrap())?;
    }

    Ok(ret)
}

struct Env {
    pub pokemon_names: HashMap<u32, String>,
    pub pokemon_moves: HashMap<u32, Vec<u32>>,
    pub pokemon_abilities: HashMap<u32, Vec<u32>>,
    pub move_names: HashMap<u32, String>,
    pub ability_names: HashMap<u32, String>,
    pub items: Vec<String>,
}

impl Env {
    fn load() -> Env {
        let csv_dir = Path::new(CSV_PATH);

        println!("Loading pokemon names...");

        let mut name_rdr = csv::Reader::from_file(csv_dir.join("pokemon_species_names.csv"))
                                   .unwrap()
                                   .has_headers(true);
        let mut pokemon_names = HashMap::new();
        for s_row in name_rdr.decode::<SpeciesName>().filter_map(|n| n.ok()).filter(|n| n.lang_id == 9) {
            pokemon_names.insert(s_row.species_id, s_row.name);
        }

        println!("Loading pokemon moves...");

        let mut move_rdr = csv::Reader::from_file(csv_dir.join("pokemon_moves.csv"))
                                   .unwrap()
                                   .has_headers(true);
        let mut pokemon_moves = HashMap::new();
        for m_row in move_rdr.decode::<PokemonMove>().filter_map(|m| m.ok()).filter(|m| m.group_id == 16) {
            let moves = pokemon_moves.entry(m_row.pokemon_id).or_insert(Vec::new());
            moves.push(m_row.move_id);
        }

        println!("Loading move names...");

        let mut move_name_rdr = csv::Reader::from_file(csv_dir.join("move_names.csv"))
                                        .unwrap()
                                        .has_headers(true);
        let mut move_names = HashMap::new();
        for n_row in move_name_rdr.decode::<MoveName>().filter_map(|n| n.ok()).filter(|n| n.lang_id == 9) {
            move_names.insert(n_row.move_id, n_row.name);
        }

        let mut ability_name_rdr = csv::Reader::from_file(csv_dir.join("ability_names.csv"))
                                               .unwrap()
                                               .has_headers(true);
        let mut ability_stat_rdr = csv::Reader::from_file(csv_dir.join("abilities.csv"))
                                               .unwrap()
                                               .has_headers(true);

        let mut abilities = HashSet::new();
        let mut ability_names = HashMap::new();

        println!("Loading ability list...");

        for (id, _, _, is_main) in ability_stat_rdr.decode::<(u32, String, u32, u32)>()
                                                            .filter_map(|a| a.ok())
        {
            if is_main == 1 {
                abilities.insert(id);
            }
        }

        println!("Loading ability names...");

        for a_name in ability_name_rdr.decode::<MoveName>().filter_map(|a| a.ok()).filter(|n| n.lang_id == 9) {
            if abilities.contains(&a_name.move_id) {
                ability_names.insert(a_name.move_id, a_name.name);
            }
        }

        println!("Loading abilities for pokemon...");

        let mut ability_rdr = csv::Reader::from_file(csv_dir.join("pokemon_abilities.csv"))
                                          .unwrap()
                                          .has_headers(true);
        let mut pokemon_abilities = HashMap::new();
        for (mon_id, ability, _, _) in ability_rdr.decode::<(u32, u32, u32, u32)>().filter_map(|a| a.ok()) {
            if abilities.contains(&ability) {
                let this_abilities = pokemon_abilities.entry(mon_id).or_insert(Vec::new());
                this_abilities.push(ability);
            }
        }

        let mut item_rdr = csv::Reader::from_file(csv_dir.join("items.csv"))
                                       .unwrap()
                                       .has_headers(true);
        let mut item_name_rdr = csv::Reader::from_file(csv_dir.join("item_names.csv"))
                                            .unwrap()
                                            .has_headers(true);
        let mut items = HashSet::new();
        let mut item_names = Vec::new();

        println!("Loading item list...");

        for item_row in item_rdr.decode::<Item>().filter_map(|i| i.ok()) {
            if [3,4,5,6,7,12,13,15,17,18,19].contains(&item_row.category) {
                items.insert(item_row.item_id);
            }
        }

        println!("Loading item names...");

        for (item_id, lang_id, name) in item_name_rdr.decode::<(u32, u32, String)>().filter_map(|n| n.ok()) {
            if lang_id == 9 && items.contains(&item_id) {
                item_names.push(name);
            }
        }

        Env {
            pokemon_names: pokemon_names,
            pokemon_moves: pokemon_moves,
            pokemon_abilities: pokemon_abilities,
            move_names: move_names,
            ability_names: ability_names,
            items: item_names,
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

#[derive(RustcDecodable)]
struct Item {
    item_id: u32,
    _identifier: String,
    category: u32,
    _cost: u32,
    _fling_power: Option<u32>,
    _fling_effect_id: Option<u32>,
}
