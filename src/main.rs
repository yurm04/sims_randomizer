use floem::{
    event::EventListener, kurbo::Size, view::View, views::Decorators, window::WindowConfig,
    Application,
};

mod ui {
    pub mod app_view;
    pub mod list_item;
}

mod trait_randomizer {
    pub mod randomize;
}

use crate::trait_randomizer::randomize::randomize;
use crate::ui::app_view::app_view;

fn main() {
    let randomized_traits = randomize();
    println!("{:?}", randomized_traits);
    let view = app_view();
    Application::new()
        .window(
            move |_| {
                let id = view.id();
                view.on_event_stop(EventListener::KeyUp, move |e| {
                    if let floem::event::Event::KeyUp(e) = e {
                        if e.key.logical_key
                            == floem::keyboard::Key::Named(floem::keyboard::NamedKey::Escape)
                        {
                            id.inspect();
                        }
                    }
                })
            },
            Some(
                WindowConfig::default()
                    .size(Size::new(500.0, 300.0))
                    .title("Yurms!"),
            ),
        )
        .run();
}

/*
 * TODO
 *
 * - move our dice logic to its own module
 * - create a UI module
 * - move all logic into its own struct
 * - build the UI
 */

// use rand::{seq::SliceRandom, thread_rng};
// use serde::Deserialize;
// use std::{
//     fmt::{Display, Formatter, Result},
//     fs,
// };
// use toml::from_str;

// use strum::IntoEnumIterator; // 0.17.1
// use strum_macros::EnumIter; // 0.17.1

// #[derive(Debug, Deserialize, Copy, Clone)]
// enum GamePacks {
//     BaseGameTraits,
//     GardenStuffTraits,
//     ChefHustleTraits,
// }

// #[derive(Debug, Copy, Clone)]
// enum GameTraits {
//     BaseGameTraits,
//     GardenStuffTraits,
//     ChefHustleTraits,
// }

// // enums are always at root level
// #[derive(Debug, Copy, Clone, EnumIter, Deserialize)]
// enum AllTraits {
//     BaseGameCheerful,
//     BaseGameMean,
//     BaseGameAthletic,
//     BaseGameFunny,
//     BaseGameGenius,
//     BaseGameGlutton,
//     BaseGameLactoseIntolerant,
//     GardenStuffGreenthumb,
//     GardenStuffPatient,
//     GardenStuffOutdoorsy,
//     ChefHustleFoodie,
//     ChefHustleSnob,
//     ChefHustleClean,
// }

// #[derive(Debug, Deserialize)]
// struct ConfigFile {
//     packs: Vec<GamePacks>,
//     #[serde(default)]
//     excluded_traits: Vec<String>,
// }

// impl Default for ConfigFile {
//     fn default() -> Self {
//         Self {
//             packs: vec![GamePacks::BaseGameTraits],
//             excluded_traits: vec![],
//         }
//     }
// }

// impl Display for AllTraits {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{:?}", self)
//     }
// }

// fn read_config_file() -> ConfigFile {
//     let config =
//         fs::read_to_string("./config.toml").unwrap_or(String::from("packs=[\"BaseGameTraits\"]"));
//     from_str(&config).unwrap()
// }

// fn main() {
//     // let mut range = thread_rng();
//     // let mut personality_traits = vec![Cheerful, Mean, Athletic, Funny, Genius];

//     // personality_traits.shuffle(&mut range);

//     // let first_three = vec![
//     //     personality_traits.pop().unwrap_or_else(|| Mean),
//     //     personality_traits.pop().unwrap_or(Mean),
//     //     personality_traits.pop().unwrap_or(Mean),
//     // ];

//     // println!("{:?}", first_three)

//     let config = read_config_file();

//     let mut personality_traits: Vec<AllTraits> = Vec::new();

//     println!("{:?}", config);
//     config.packs.iter().for_each(|p| {
//         let pack = format!("{:?}", p).replace("Traits", "");

//         for t in AllTraits::iter() {
//             let name = format!("{t}");
//             if name.starts_with(&pack) && !config.excluded_traits.contains(&name.replace(&pack, ""))
//             {
//                 personality_traits.push(t);
//             }
//         }
//     });

//     println!("{:?}", personality_traits);
//     let mut range = thread_rng();

//     personality_traits.shuffle(&mut range);

//     let first_three = vec![
//         personality_traits
//             .pop()
//             .unwrap_or_else(|| AllTraits::BaseGameMean),
//         personality_traits.pop().unwrap_or(AllTraits::BaseGameMean),
//         personality_traits.pop().unwrap_or(AllTraits::BaseGameMean),
//     ];

//     println!("{:?}", first_three)

//     // println!("{:?}", personality_traits);
// }
