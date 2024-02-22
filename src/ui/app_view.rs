use floem::{
    reactive::{create_rw_signal, RwSignal},
    view::View,
    views::{dyn_container, h_stack, label, v_stack, Decorators},
    widgets::checkbox,
};

use crate::ui::list_item::list_item;

pub fn app_view() -> impl View {
    v_stack((
        label(|| "Game Packs").style(|s| s.font_size(13.0).margin_bottom(10)),
        list_item(String::from("Base Game")).style(|s| s.gap(5, 0).items_center()),
    ))
    .style(|s| s.width_full().margin(5))
}
