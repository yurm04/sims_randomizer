use floem::{
    peniko::Color,
    reactive::{create_rw_signal, provide_context, use_context, RwSignal},
    style::Style,
    view::View,
    views::{dyn_stack, h_stack, label, scroll, v_stack, v_stack_from_iter, Decorators},
    widgets::button,
};
use strum::IntoEnumIterator; // 0.17.1

use crate::trait_randomizer::randomize::{randomize, AllTraits, GamePacks};
use crate::ui::list_item::list_item;

/**
 * Implement trait updating similar to gamepacks with static vec and signal vec
 *
 * create onclick handle function for traits
 *
 * profit!
 */

pub type PackSignals = RwSignal<Vec<(GamePacks, bool)>>;
pub type SelectedGamePacks = RwSignal<Vec<GamePacks>>;

fn handle_select_game_pack(game_pack: GamePacks, selected: bool) {
    let selected_game_packs =
        use_context::<SelectedGamePacks>().expect("Requires selected GamePacks context");

    if selected {
        selected_game_packs.update(|game_packs| game_packs.push(game_pack));
    } else {
        selected_game_packs.update(|game_packs| {
            *game_packs = game_packs
                .iter()
                .filter(|selected_game_pack| **selected_game_pack != game_pack)
                .map(|game_pack| *game_pack)
                .collect::<Vec<GamePacks>>();
        })
    }
}

pub fn app_view() -> impl View {
    let output = create_rw_signal(String::new());
    let game_packs = GamePacks::iter().collect::<Vec<GamePacks>>();
    let selected_game_packs: SelectedGamePacks = create_rw_signal(Vec::new());

    provide_context(selected_game_packs);

    // TODO turn this into a tuple for states: is_visible and is_checked. Do it like packs
    // let trait_list = create_rw_signal(AllTraits::iter().collect::<Vec<AllTraits>>());

    h_stack((
        v_stack((
            label(|| "Game Packs").style(|s| s.font_size(13.0).margin_bottom(10)),
            v_stack_from_iter(
                game_packs
                    .iter()
                    .map(|game_pack| list_item(*game_pack, |_, _| {})),
            )
            .style(|s| s.gap(0, 5)),
            button(|| "Shuffle traits").on_click_cont(move |_| {
                // output.set(format!(
                //     "{:#?}",
                //     randomize(
                //         packs
                //             .get()
                //             .iter()
                //             .filter(|(_, is_checked)| *is_checked)
                //             .map(|(label, _)| *label)
                //             .collect::<Vec<GamePacks>>()
                //     )
                // ));
            }),
        ))
        .style(|s| s.width_full().margin(5)),
        // scroll(
        //     // TODO bug: the dyn stack is not
        //     // TODO rethink the architecture of this. Triggers maybe?
        //     dyn_stack(
        //         move || {
        //             // let packs = packs.clone();
        //             // trait_list.get().into_iter().filter(move |trait_name| {
        //             //     let trait_name = trait_name.clone();
        //             //     packs.iter().any(move |(pack_name, checked)| {
        //             //         checked.get()
        //             //             && trait_name
        //             //                 .to_string()
        //             //                 .starts_with(&pack_name.to_string().clone())
        //             //     })
        //             // })
        //             trait_list
        //                 .get()
        //                 .into_iter()
        //                 .filter(move |game_trait| {
        //                     packs
        //                         .get()
        //                         .iter()
        //                         .find_map(|pack| {
        //                             if game_trait.to_string().starts_with(&pack.0.to_string()) {
        //                                 Some(pack.1)
        //                             } else {
        //                                 None
        //                             }
        //                         })
        //                         .unwrap_or(false)
        //                 })
        //                 .collect::<Vec<AllTraits>>()
        //         },
        //         move |trait_name| *trait_name,
        //         move |trait_name| list_item(trait_name, |_, _| true),
        //     )
        //     .style(|s| s.flex_col().gap(0, 5)),
        // )
        // .style(|s| s.height_full().class(scroll::Handle, scrollbar_styles)),
        label(move || output.get()),
    ))
}

fn scrollbar_styles(s: Style) -> Style {
    const SCROLL_BAR_COLOR: Color = Color::rgb8(41, 98, 218);

    s.background(SCROLL_BAR_COLOR.with_alpha_factor(0.3))
        .hover(|s| s.background(SCROLL_BAR_COLOR.with_alpha_factor(0.7)))
        .active(|s| s.background(SCROLL_BAR_COLOR))
        .set(scroll::Thickness, 5.0)
}
