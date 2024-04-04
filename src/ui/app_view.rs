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
use crate::ui::heading::heading;
use crate::ui::list_item::list_item;

/**
 * Implement trait updating similar to gamepacks with static vec and signal vec
 *
 * create onclick handle function for traits
 *
 * profit!
 */

pub type SelectedGamePacks = RwSignal<Vec<GamePacks>>;
type GameTraitsStatus = Vec<(AllTraits, bool)>;
pub type EnabledGameTraits = RwSignal<GameTraitsStatus>;

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
                .map(|game_trait| (*game_trait, selected))
                .collect::<GameTraitsStatus>();

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
                .collect::<GameTraitsStatus>();

            *game_traits = mapped_game_traits;
        })
    }
}

fn handle_select_game_trait(game_trait: AllTraits, selected: bool) {
    let enabled_game_traits =
        use_context::<EnabledGameTraits>().expect("Requires enabled AllTraits context");

    enabled_game_traits.update(|game_traits| {
        *game_traits = game_traits
            .iter()
            .map(|current_game_trait| {
                if current_game_trait.0 == game_trait {
                    (current_game_trait.0, selected)
                } else {
                    *current_game_trait
                }
            })
            .collect::<GameTraitsStatus>();
    });
}

pub fn app_view() -> impl View {
    let output: RwSignal<Vec<String>> = create_rw_signal(Vec::new());
    let game_packs = GamePacks::iter().collect::<Vec<GamePacks>>();
    let selected_game_packs: SelectedGamePacks = create_rw_signal(vec![GamePacks::BaseGame]);
    let default_base_game_traits = AllTraits::iter()
        .filter(|game_trait_name| game_trait_name.to_string().starts_with("BaseGame"))
        .map(|game_traits| (game_traits, true))
        .collect::<GameTraitsStatus>();
    let enabled_game_traits: EnabledGameTraits = create_rw_signal(default_base_game_traits);

    provide_context(selected_game_packs);
    provide_context(enabled_game_traits);

    h_stack((
        v_stack((
            heading(String::from("Game Packs")),
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
                output.set(randomize(
                    enabled_game_traits
                        .get()
                        .iter()
                        .filter(|(_, selected)| *selected)
                        .map(|(game_trait_name, _)| *game_trait_name)
                        .collect::<Vec<AllTraits>>(),
                ));
            }),
        ))
        .style(|s| s.width_full().gap(0, 5)),
        v_stack((
            heading(String::from("Game Traits")),
            scroll(
                dyn_stack(
                    move || enabled_game_traits.get(),
                    move |trait_name| *trait_name,
                    move |(trait_name, selected)| {
                        list_item(trait_name, handle_select_game_trait, selected, false)
                    },
                )
                .style(|s| s.flex_col().gap(0, 5)),
            )
            .style(|s| s.height_full().class(scroll::Handle, scrollbar_styles)),
        )),
        dyn_stack(
            move || output.get(),
            move |randomized_traits| randomized_traits.clone(),
            move |trait_name| label(move || trait_name.clone()),
        )
        .style(|s| s.flex_col()),
    ))
    .style(|s| s.gap(15, 0).margin(10))
}

fn scrollbar_styles(s: Style) -> Style {
    const SCROLL_BAR_COLOR: Color = Color::rgb8(41, 98, 218);

    s.background(SCROLL_BAR_COLOR.with_alpha_factor(0.3))
        .hover(|s| s.background(SCROLL_BAR_COLOR.with_alpha_factor(0.7)))
        .active(|s| s.background(SCROLL_BAR_COLOR))
        .set(scroll::Thickness, 5.0)
}
