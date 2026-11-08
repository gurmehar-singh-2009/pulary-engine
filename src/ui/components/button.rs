use gpui::{IntoElement, ParentElement, Styled, div, px, rgb};

use crate::ui::components::colors::THEME;

pub fn btn_neutral(text: &str) -> impl IntoElement {
    let text = text.to_string();

    div()
        .text_align(gpui::TextAlign::Center)
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .child(text)
        .bg(rgb(THEME.accent))
        .text_color(rgb(THEME.text_c1))
        .rounded(px(12.))
        .p(px(12.))
        .border(px(2.))
        .border_color(rgb(THEME.border))
}
