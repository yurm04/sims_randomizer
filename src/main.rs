use std::collections::HashMap;
extern crate rand;
extern crate serde_json;

use rand::seq::index::sample;

// fn main() {
//     // let mut rng = rand::thread_rng();
//     // let arr: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     // let sample: Vec<u8> = sample(&mut rng, arr.len(), 3)
//     //     .iter()
//     //     .map(|i| arr[i])
//     //     .collect();
// }
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=494d4533124c8d06c5214fcab43772e1

use serde::Deserialize;

#[derive(Debug)]
struct GameWithRandomTraits {
    name: String,
    traits: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct GameWithAllTraits {
    games: HashMap<String, Vec<String>>,
}

fn main() {
    let game_data = load_game_data();
    println!(
        "{:#?}",
        randomize_traits(&game_data, String::from("The Sims 4"), 5)
    );
}

fn randomize_traits(
    all_traits: &GameWithAllTraits,
    game: String,
    amount: usize,
) -> GameWithRandomTraits {
    let mut range = rand::thread_rng();
    let traits = all_traits.games.get(&game).unwrap();
    let random_traits = sample(&mut range, traits.len(), amount)
        .iter()
        .map(|t| traits[t].to_string())
        .collect();
    GameWithRandomTraits {
        name: game,
        traits: random_traits,
    }
}

fn load_game_data() -> GameWithAllTraits {
    let data = include_str!("./data/game_traits.json");
    serde_json::from_str(data).unwrap_or_else(|error| {
        panic!(
            "JSON parsing error encountered for: \"game_traits.json\"\nError: {}",
            error
        )
    })
}

#[test]
fn randomize_traits_works() {
    let mut test_traits = HashMap::new();
    test_traits.insert(
        String::from("test1"),
        vec![
            String::from("test1 - trait1"),
            String::from("test1 - trait2"),
            String::from("test1 - trait3"),
        ],
    );
    test_traits.insert(
        String::from("test2"),
        vec![
            String::from("test2 - trait1"),
            String::from("test2 - trait2"),
            String::from("test2 - trait3"),
        ],
    );
    test_traits.insert(
        String::from("test3"),
        vec![
            String::from("test3 - trait1"),
            String::from("test3 - trait2"),
            String::from("test3 - trait3"),
        ],
    );
    let test_data = GameWithAllTraits { games: test_traits };

    let result = randomize_traits(&test_data, String::from("test2"), 1);
    assert!(test_data
        .games
        .get("test2")
        .unwrap()
        .contains(&result.traits[0]));
}
