use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::{
    fmt::{format, Display, Formatter, Result},
    fs,
};
use toml::from_str;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, Deserialize, Copy, Clone, EnumIter)]
pub enum GamePacks {
    BaseGame,
    GardenStuff,
    ChefHustle,
}

// enums are always at root level
#[derive(Debug, Copy, Clone, EnumIter, Deserialize)]
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
    packs: Vec<GamePacks>,
    #[serde(default)]
    excluded_traits: Vec<String>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            packs: vec![GamePacks::BaseGame],
            excluded_traits: vec![],
        }
    }
}

impl Display for AllTraits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

fn read_config_file() -> ConfigFile {
    let config =
        fs::read_to_string("./config.toml").unwrap_or(String::from("packs=[\"BaseGameTraits\"]"));
    from_str(&config).unwrap()
}

pub fn randomize() -> Vec<String> {
    // let mut range = thread_rng();
    // let mut personality_traits = vec![Cheerful, Mean, Athletic, Funny, Genius];

    // personality_traits.shuffle(&mut range);

    // let first_three = vec![
    //     personality_traits.pop().unwrap_or_else(|| Mean),
    //     personality_traits.pop().unwrap_or(Mean),
    //     personality_traits.pop().unwrap_or(Mean),
    // ];

    // println!("{:?}", first_three)

    let config = read_config_file();

    let mut personality_traits: Vec<String> = Vec::new();

    config.packs.iter().for_each(|p| {
        let pack = format!("{:?}", p);

        for t in AllTraits::iter() {
            let name = format!("{t}");
            if name.starts_with(&pack) && !config.excluded_traits.contains(&name.replace(&pack, ""))
            {
                let trait_name = format!("{name}").replace(&pack, "");
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
