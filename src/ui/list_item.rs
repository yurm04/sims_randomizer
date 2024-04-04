use std::sync::Arc;

use floem::{
    reactive::create_rw_signal,
    view::View,
    views::{h_stack, label, Decorators},
    widgets::checkbox,
};
use std::fmt::Display;

pub fn list_item<T: Display + PartialEq + Clone + Copy + 'static>(
    item_label: T,
    on_click: impl Fn(T, bool) + 'static,
    selected: bool,
    disabled: bool,
) -> impl View {
    let selected = create_rw_signal(selected);

    let on_click = Arc::new(on_click);

    h_stack((
        checkbox(move || selected.get())
            .on_update({
                let on_click = Arc::clone(&on_click);
                move |_| {
                    selected.set(!selected.get());
                    on_click(item_label, selected.get());
                }
            })
            .disabled(move || disabled),
        label(move || item_label.to_string()).on_click_cont({
            let on_click = Arc::clone(&on_click);
            move |_| {
                if !disabled {
                    selected.set(!selected.get());
                    on_click(item_label, selected.get());
                }
            }
        }),
    ))
    .style(|s| s.gap(5.0, 0).items_center())
}
