use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::{
    fmt::{Display, Formatter, Result},
    fs,
};
use toml::from_str;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::{Display, EnumIter}; // 0.17.1

#[derive(Debug, Deserialize, Copy, Clone, EnumIter, PartialEq, Eq)]
pub enum GamePacks {
    BaseGame,
    GardenStuff,
    ChefHustle,
}

// enums are always at root level
#[derive(Debug, Copy, Clone, EnumIter, Deserialize, Eq, PartialEq, Hash)]
pub enum AllTraits {
    BaseGameCheerful,
    BaseGameMean,
    BaseGameAthletic,
    BaseGameFunny,
    BaseGameGenius,
    BaseGameGlutton,
    BaseGameLactoseIntolerant,
    GardenStuffGreenthumb,
    GardenStuffPatient,
    GardenStuffOutdoorsy,
    ChefHustleFoodie,
    ChefHustleSnob,
    ChefHustleClean,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    _packs: Vec<GamePacks>,
    #[serde(default)]
    excluded_traits: Vec<String>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            _packs: vec![GamePacks::BaseGame],
            excluded_traits: vec![],
        }
    }
}

impl Display for AllTraits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for GamePacks {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

fn read_config_file() -> ConfigFile {
    let config =
        fs::read_to_string("./config.toml").unwrap_or(String::from("packs=[\"BaseGameTraits\"]"));
    from_str(&config).unwrap()
}

pub fn randomize(enabled_game_traits: Vec<AllTraits>) -> Vec<String> {
    println!("{:?}", enabled_game_traits);
    let mut personality_traits = enabled_game_traits
        .iter()
        .map(|game_trait| {
            let mut game_trait_name = format!("{:?}", game_trait);

            for game_pack in GamePacks::iter() {
                let game_pack_name = format!("{game_pack}");
                if game_trait_name.starts_with(&game_pack_name) {
                    game_trait_name = game_trait_name.replace(&game_pack_name, "");
                }
            }

            game_trait_name
        })
        .collect::<Vec<String>>();

    let mut range = thread_rng();

    personality_traits.shuffle(&mut range);

    let first_three = vec![
        personality_traits.pop().unwrap(),
        personality_traits.pop().unwrap(),
        personality_traits.pop().unwrap(),
    ];

    first_three
}
