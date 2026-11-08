use gpui::{IntoElement, Styled, div, px, relative, rgb};

use crate::ui::components::colors::THEME;

pub fn node_explorer_ui(window: &mut gpui::Window) -> impl IntoElement {
    let viewport_size = window.viewport_size();
    let max_h = viewport_size.height;

    div()
        .w(relative(0.25))
        .h_full()
        .flex_none()
        .border_r_1()
        .bg(rgb(THEME.foreground))
        .border_color(rgb(THEME.border))
}
