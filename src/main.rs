// const PACKS: [&str; 13] = [
//     "The Sims 4",
//     "Spa Day",
//     "City Living",
//     "Eco Lifestyle",
//     "Snowy Escape",
//     "Cottage Living",
//     "Cats & Dogs",
//     "Island Living",
//     "Get Together",
//     "High School Years",
//     "Outdoor Retreat",
//     "StrangerVille",
//     "Get Famous"
// ];

use std::{collections::HashMap, vec};

fn main() {
    let mut games = HashMap::new();

    // base game traits
    games.insert("base", vec![
        "Active",
        "Cheerful",
        "Creative",
        "Genius",
        "Gloomy",
        "Goofball",
        "Hot-Headed",
        "Romantic",
        "Self-Assured",
        "Art Lover",
        "Bookworm",
        "Foodie",
        "Geek",
        "Music Lover",
        "Perfectionist",
        "Ambitious",
        "Childish",
        "Clumsy",
        "Erratic",
        "Glutton",
        "Kleptomaniac",
        "Lazy",
        "Loves Outdoors",
        "Materialistic",
        "Neat",
        "Slob",
        "Snob",
        "Vegetarian",
        "Bro",
        "Evil",
        "Family-oriented",
        "Good",
        "Hates Children",
        "Jealous",
        "Loner",
        "Loyal",
        "Mean",
        "Noncommittal",
        "Outgoing"
    ]);
    // spa day game traits
    games.insert("Spa Day", vec![
        "High Maintenance"
    ]);
    // city living
    games.insert("City Living", vec![
        "Unflirty"
    ]);
    games.insert("Eco Lifestyle", vec![
        "Maker",
        "Freegan",
        "Green Fiend",
        "Recycle Disciple"
    ]);
    games.insert("Snowy Escape", vec![
        "Adventurous",
        "Proper"
    ]);
    games.insert("Cottage Living", vec![
        "Animal Enthusiast",
        "Lactose Intolerant"
    ]);
    games.insert("Cats & Dogs", vec![
        "Cat Lover",
        "Dog Lover"
    ]);
    games.insert("Island Living", vec![
        "Child of the Islands",
        "Child of the Ocean"
    ]);
    games.insert("Get Together", vec![
        "Dance Machine",
        "Insider"
    ]);
    games.insert("High School Years", vec![
        "Overachiever",
        "Party Animal",
        "Socially Awkward"
    ]);
    games.insert("Outdoor Retreat", vec![
        "Squeamish"
    ]);
    games.insert("StrangerVille", vec![
        "Paranoid"
    ]);
    games.insert("Get Famous", vec![
        "Self-Absorbed"
    ]);



}
