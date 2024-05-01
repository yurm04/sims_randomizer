use floem::{
    event::EventListener, kurbo::Size, view::View, views::Decorators, window::WindowConfig,
    Application,
};

mod ui {
    pub mod app_view;
    pub mod heading;
    pub mod list_item;
}

mod trait_randomizer {
    pub mod randomize;
}

use crate::ui::app_view::app_view;

fn main() {
    let view = app_view();
    Application::new()
        .window(
            move |_| {
                let id = view.id();
                view.on_event_stop(EventListener::KeyUp, move |e| {
                    if let floem::event::Event::KeyUp(e) = e {
                        if e.key.logical_key
                            == floem::keyboard::Key::Named(floem::keyboard::NamedKey::Escape)
                        {
                            id.inspect();
                        }
                    }
                })
            },
            Some(
                WindowConfig::default()
                    .size(Size::new(550.0, 300.0))
                    .title("Yurms!"),
            ),
        )
        .run();
}
