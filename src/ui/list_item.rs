use floem::{
    reactive::{create_rw_signal, use_context},
    view::View,
    views::{h_stack, label, Decorators},
    widgets::checkbox,
};

pub fn list_item(item_label: String) -> impl View {
    // Create a reactive signal with a counter value, defaulting to 0
    let context: Option<Vec<String>> = use_context();
    println!("{:?}", context);
    let is_checked = create_rw_signal(false);

    h_stack((
        checkbox(move || is_checked.get()),
        label(move || item_label.clone()).on_click_cont(move |_| is_checked.set(!is_checked.get())),
    ))
    .style(|s| s.gap(5.0, 0).items_center())
}
