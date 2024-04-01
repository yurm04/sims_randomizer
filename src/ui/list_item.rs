use std::sync::Arc;

use floem::{
    reactive::{create_rw_signal, use_context, RwSignal},
    view::View,
    views::{h_stack, label, Decorators},
    widgets::checkbox,
};
use std::fmt::Display;

use crate::ui::app_view::PackSignals;

pub fn list_item<T: Display + PartialEq + Clone + Copy + 'static>(
    item_label: T,
    on_click: impl Fn(T, bool) + 'static,
) -> impl View {
    let selected = create_rw_signal(false);
    let checkbox_copy = item_label.clone();
    let click_label_checkbox_copy = item_label.clone();

    let on_click = Arc::new(on_click);

    h_stack((
        checkbox(move || selected.get())
            .on_update({
                let on_click = Arc::clone(&on_click);
                let checkbox_copy = checkbox_copy.clone();
                move |_| {
                    selected.set(!selected.get());
                    on_click(checkbox_copy, selected.get());
                }
            })
            .disabled(move || false),
        label(move || item_label.to_string()).on_click_cont({
            let on_click = Arc::clone(&on_click);
            let click_label_checkbox_copy = click_label_checkbox_copy.clone();
            move |_| {
                selected.set(!selected.get());
                on_click(click_label_checkbox_copy, selected.get());
            }
        }),
    ))
    .style(|s| s.gap(5.0, 0).items_center())
}
