use floem::{
    reactive::{create_rw_signal, RwSignal},
    view::View,
    views::{container, dyn_container, h_stack, label, v_stack, Container, Decorators},
    widgets::{checkbox, list},
};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use crate::trait_randomizer::randomize::GamePacks;
use crate::ui::list_item::list_item;

fn get_game_packs_vec() -> Vec<String> {
    GamePacks::iter().map(|p| format!("{:?}", p)).collect()
}

pub fn app_view() -> impl View {
    v_stack((
        label(|| "Game Packs").style(|s| s.font_size(13.0).margin_bottom(10)),
        list(
            get_game_packs_vec()
                .iter()
                .map(|label| list_item(String::from(label))),
        ),
    ))
    .style(|s| s.width_full().margin(5))
}
