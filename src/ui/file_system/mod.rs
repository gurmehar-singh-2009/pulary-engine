use std::{ops::Add, sync::atomic::Ordering};

use gpui::{
    AnyElement, InteractiveElement, IntoElement, ParentElement, Styled, div,
    prelude::FluentBuilder, px, relative, rgb,
};

mod entry;

use crate::{
    scripting::file_system::{Entry, FileSystem},
    ui::{
        components::colors::THEME,
        icons::{Icon, icon},
    },
};

pub fn node_explorer_ui(
    file_system: &mut FileSystem,
    window: &mut gpui::Window,
) -> impl IntoElement {
    let viewport_size = window.viewport_size();

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
        .child(file_tree(&mut file_system.entries))
}

fn file_tree(entries: &mut [Entry]) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .text_color(rgb(THEME.text_c1))
        .children(entries.iter_mut().map(|entry| {
            let expanded = entry.expanded.clone();

            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(24.))
                        .flex()
                        .items_center()
                        .gap(px(12.))
                        .px(px(8.))
                        .text_size(px(14.))
                        .text_color(rgb(THEME.text_c3))
                        .on_mouse_up(gpui::MouseButton::Left, move |_, w, _| {
                            expanded.fetch_xor(true, Ordering::Relaxed);
                            w.refresh();
                        })
                        .child(if entry.is_file {
                            icon(Icon::File)
                        } else {
                            icon(Icon::Folder)
                        })
                        .child(entry.name.to_string())
                )
                .when(
                    entry.expanded.load(Ordering::Relaxed) && !entry.children.is_empty(),
                    |el| el.child(div().pl(px(12.)).child(file_tree(&mut entry.children))),
                )
        }))
        .into_any_element()
}
