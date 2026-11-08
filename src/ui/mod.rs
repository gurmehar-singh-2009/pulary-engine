use gpui::{IntoElement, ParentElement, Render, Styled, div, px, relative, rgb};

use crate::ui::{components::colors::THEME, node_explorer::node_explorer_ui};

mod components;
mod icons;
mod node_explorer;

pub struct Ui {}

impl Render for Ui {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        _cx: &mut gpui::prelude::Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .bg(rgb(THEME.background))
            .text_color(rgb(THEME.text_c1))
            .child(node_explorer_ui(window))
            .child(
                div()
                    .w(relative(0.75))
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(top_bar()),
            )
    }
}

fn top_bar() -> impl IntoElement {
    div()
        .w_full()
        .h(px(32.))
        .bg(rgb(THEME.foreground))
        .flex()
        .flex_row()
        .children([top_bar_tab("Editor"), top_bar_tab("Visualizer")])
}

fn top_bar_tab(name: &str) -> impl IntoElement {
    let name = name.to_string();

    div()
        .flex()
        .h(px(32.))
        .px(px(8.))
        .bg(rgb(THEME.selection))
        .text_color(rgb(THEME.text_c2))
        .text_size(px(12.))
        .border_r_1()
        .border_color(rgb(THEME.border))
        .items_center()
        .justify_center()
        .child(name)
}
