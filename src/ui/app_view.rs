use floem::{
    reactive::{create_rw_signal, RwSignal},
    view::View,
    views::{h_stack, label, v_stack, v_stack_from_iter, Decorators},
    widgets::button,
};
use strum::IntoEnumIterator; // 0.17.1

use crate::trait_randomizer::randomize::{randomize, GamePacks};
use crate::ui::list_item::list_item;

pub fn app_view() -> impl View {
    let output = create_rw_signal(String::new());
    let packs = GamePacks::iter()
        .map(|label| (label, create_rw_signal(false)))
        .collect::<Vec<(GamePacks, RwSignal<bool>)>>();

    h_stack((
        v_stack((
            label(|| "Game Packs").style(|s| s.font_size(13.0).margin_bottom(10)),
            v_stack_from_iter(
                packs
                    .iter()
                    .map(|(label, is_checked)| list_item(format!("{label}"), *is_checked)),
            )
            .style(|s| s.gap(0, 5)),
            button(|| "Shuffle traits").on_click_cont(move |_| {
                output.set(format!(
                    "{:#?}",
                    randomize(
                        packs
                            .iter()
                            .filter(|(_, is_checked)| is_checked.get())
                            .map(|(label, _)| *label)
                            .collect::<Vec<GamePacks>>()
                    )
                ));
            }),
        ))
        .style(|s| s.width_full().margin(5)),
        label(move || output.get()),
    ))
}
