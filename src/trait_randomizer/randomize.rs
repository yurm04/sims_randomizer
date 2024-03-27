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

pub fn randomize(selected_packs: Vec<GamePacks>) -> Vec<String> {
    let config = read_config_file();

    let mut personality_traits: Vec<String> = Vec::new();

    selected_packs.iter().for_each(|p| {
        let pack = format!("{:?}", p);

        for t in AllTraits::iter() {
            let name = format!("{t}");
            if name.starts_with(&pack) && !config.excluded_traits.contains(&name.replace(&pack, ""))
            {
                let trait_name = name.replace(&pack, "");
                personality_traits.push(trait_name);
            }
        }
    });

    let mut range = thread_rng();

    personality_traits.shuffle(&mut range);

    let first_three = vec![
        personality_traits
            .pop()
            .unwrap_or_else(|| format!("{:?}", AllTraits::BaseGameMean).replace("BaseGame", "")),
        personality_traits
            .pop()
            .unwrap_or(format!("{:?}", AllTraits::BaseGameMean).replace("BaseGame", "")),
        personality_traits
            .pop()
            .unwrap_or(format!("{:?}", AllTraits::BaseGameMean).replace("BaseGame", "")),
    ];

    first_three
}
