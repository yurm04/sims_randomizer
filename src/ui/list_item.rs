use floem::{
    reactive::RwSignal,
    view::View,
    views::{h_stack, label, Decorators},
    widgets::checkbox,
};

pub fn list_item(item_label: String, is_checked: RwSignal<bool>) -> impl View {
    // Create a reactive signal with a counter value, defaulting to 0
    // let context: Option<Vec<String>> = use_context();
    let is_base = item_label.contains("Base");

    if is_base {
        is_checked.set(true);
    }

    h_stack((
        checkbox(move || is_checked.get()).disabled(move || is_base),
        label(move || item_label.clone()).on_click_cont(move |_| {
            if !is_base {
                is_checked.set(!is_checked.get());
            }
        }),
    ))
    .style(|s| s.gap(5.0, 0).items_center())
}
