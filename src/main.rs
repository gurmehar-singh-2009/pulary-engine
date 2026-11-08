#![feature(default_field_values)]

use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, px, size};

use gpui::AppContext;

use crate::ui::Ui;

mod scripting;
mod ui;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Ui {}),
        )
        .unwrap();
    });
}
