use floem::{
    peniko::Color,
    reactive::{create_rw_signal, RwSignal},
    style::Style,
    view::View,
    views::{dyn_stack, h_stack, label, scroll, v_stack, v_stack_from_iter, Decorators},
    widgets::button,
};
use strum::IntoEnumIterator; // 0.17.1

use crate::trait_randomizer::randomize::{randomize, AllTraits, GamePacks};
use crate::ui::list_item::list_item;

pub fn app_view() -> impl View {
    let output = create_rw_signal(String::new());
    let packs = GamePacks::iter()
        .map(|label| (label, create_rw_signal(false)))
        .collect::<Vec<(GamePacks, RwSignal<bool>)>>();
    let trait_list = create_rw_signal(AllTraits::iter().collect::<Vec<AllTraits>>());

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
        scroll(
            dyn_stack(
                move || trait_list.get(),
                move |trait_name| *trait_name,
                move |trait_name| list_item(format!("{trait_name}"), create_rw_signal(true)),
            )
            .style(|s| s.flex_col().gap(0, 5)),
        )
        .style(|s| s.height_full().class(scroll::Handle, scrollbar_styles)),
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
