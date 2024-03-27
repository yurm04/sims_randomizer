use floem::{
    reactive::{create_rw_signal, use_context, RwSignal},
    view::View,
    views::{h_stack, label, Decorators},
    widgets::checkbox,
};
use std::fmt::Display;

use crate::ui::app_view::PackSignals;

pub fn list_item<T: Display + PartialEq + Clone + 'static>(
    item_label: T,
    on_click: impl Fn(T, bool) + 'static,
) -> impl View {
    let selected = create_rw_signal(false);
    let checkbox_copy = item_label.clone();

    h_stack((
        checkbox(move || selected.get())
            .on_update(move |_| {
                selected.set(!selected.get());
                on_click(checkbox_copy.clone(), selected.get());
            })
            .disabled(move || false),
        label(move || item_label.to_string()).on_click_cont(move |_| {
            // on_click(checkbox_copy.clone(), selected.get());
            // TODO get labe to work if you click on it, should check the box
        }),
    ))
    .style(|s| s.gap(5.0, 0).items_center())
}
