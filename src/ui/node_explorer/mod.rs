use gpui::{IntoElement, ParentElement, Styled, div, px, relative, rgb};

use crate::ui::{
    components::colors::THEME,
    icons::{Icon, icon},
};

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
        .flex()
        .flex_col()
        .child(
            div()
                .w_full()
                .h(px(32.))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px(px(8.))
                .border_b_1()
                .border_color(rgb(THEME.border))
                .child(icon(Icon::Folder))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.))
                        .children([icon(Icon::Filter), icon(Icon::Copy), icon(Icon::Trash)]),
                ),
        )
}
