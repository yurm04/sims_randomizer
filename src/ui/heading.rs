use floem::{
    view::View,
    views::{label, Decorators},
};

pub fn heading(content: String) -> impl View {
    label(move || content.clone()).style(|s| s.font_size(15.0).font_bold().margin_bottom(10))
}
