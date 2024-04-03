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
pub type EnabledGameTraits = RwSignal<Vec<AllTraits>>;
// TODO
// pub type EnabledGameTraits = RwSignal<Vec<(AllTraits, bool)>>;

fn handle_select_game_pack(game_pack: GamePacks, selected: bool) {
    let game_pack_traits = AllTraits::iter().collect::<Vec<AllTraits>>();
    let selected_game_packs =
        use_context::<SelectedGamePacks>().expect("Requires selected GamePacks context");
    let enabled_game_traits =
        use_context::<EnabledGameTraits>().expect("Requires enabled AllTraits context");

    if selected {
        selected_game_packs.update(|game_packs| game_packs.push(game_pack));

        enabled_game_traits.update(|game_traits| {
            let mut mapped_game_traits = game_pack_traits
                .iter()
                .filter(|game_trait| {
                    let game_trait_name = format!("{:?}", game_trait);
                    let game_pack_name = format!("{:?}", game_pack);

                    game_trait_name.contains(&game_pack_name)
                })
                .map(|game_trait| *game_trait)
                .collect::<Vec<AllTraits>>();

            game_traits.append(&mut mapped_game_traits);
        });
    } else {
        selected_game_packs.update(|game_packs| {
            *game_packs = game_packs
                .iter()
                .filter(|selected_game_pack| **selected_game_pack != game_pack)
                .map(|game_pack| *game_pack)
                .collect::<Vec<GamePacks>>();
        });

        enabled_game_traits.update(|game_traits| {
            let mapped_game_traits = game_traits
                .iter()
                .filter(|game_trait| {
                    let game_trait_name = format!("{:?}", game_trait);
                    let game_pack_name = format!("{:?}", game_pack);

                    !game_trait_name.contains(&game_pack_name)
                })
                .map(|game_trait| *game_trait)
                .collect::<Vec<AllTraits>>();

            *game_traits = mapped_game_traits;
        })
    }
}

fn handle_select_game_trait(game_trait: AllTraits, selected: bool) {
    let enabled_game_traits =
        use_context::<EnabledGameTraits>().expect("Requires enabled AllTraits context");

    // TODO: map and set found item to `selected`
    if selected {
        enabled_game_traits.update(|game_traits| game_traits.push(game_trait));
    } else {
        enabled_game_traits.update(|game_traits| {
            *game_traits = game_traits
                .iter()
                .filter(|trait_name| **trait_name != game_trait)
                .map(|trait_name| *trait_name)
                .collect::<Vec<AllTraits>>();
        })
    }
}

pub fn app_view() -> impl View {
    let output = create_rw_signal(String::new());
    let game_packs = GamePacks::iter().collect::<Vec<GamePacks>>();
    let selected_game_packs: SelectedGamePacks = create_rw_signal(vec![GamePacks::BaseGame]);
    let default_base_game_traits = AllTraits::iter()
        .filter(|game_trait_name| game_trait_name.to_string().starts_with("BaseGame"))
        .collect::<Vec<AllTraits>>();
    let enabled_game_traits: EnabledGameTraits = create_rw_signal(default_base_game_traits);

    provide_context(selected_game_packs);
    provide_context(enabled_game_traits);

    h_stack((
        v_stack((
            label(|| "Game Packs").style(|s| s.font_size(13.0).margin_bottom(10)),
            v_stack_from_iter(game_packs.iter().map(|game_pack| {
                let is_base_game = game_pack.to_string() == String::from("BaseGame");
                list_item(
                    *game_pack,
                    handle_select_game_pack,
                    is_base_game,
                    is_base_game,
                )
            }))
            .style(|s| s.gap(0, 5)),
            button(|| "Shuffle traits").on_click_cont(move |_| {
                output.set(format!(
                    "{:#?}",
                    randomize(
                        // TODO: filter out the false (not selected traits)
                        enabled_game_traits.get()
                    )
                ));
            }),
        ))
        .style(|s| s.width_full().margin(5)),
        v_stack((dyn_stack(
            move || enabled_game_traits.get(),
            move |trait_name| *trait_name,
            // TODO: fix tuple (trait_name, _)
            move |trait_name| list_item(trait_name, handle_select_game_trait, true, false),
        )
        .style(|s| s.flex_col().gap(0, 5)),)),
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
