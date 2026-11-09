use std::ops::Deref;
use std::sync::{Arc, Mutex};

use gpui::AppContext;
use gpui::{
    App, Application, Bounds, Context, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, WindowBounds, WindowOptions, div, prelude::FluentBuilder,
    px, relative, rgb, size,
};

use crate::scripting::file_system::FileSystem;
use crate::ui::{
    components::colors::THEME,
    file_system::node_explorer_ui,
    icons::{Icon, icon},
};

mod components;
mod file_system;
mod icons;

pub use icons::Assets;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TopBarTab {
    Editor,
    Visualizer,
}

pub struct Ui {
    pub active_tab: TopBarTab,
    pub file_system: Arc<Mutex<FileSystem>>,
}

impl Render for Ui {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::prelude::Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .bg(rgb(THEME.background))
            .text_color(rgb(THEME.text_c1))
            .when(self.active_tab == TopBarTab::Editor, |el| {
                el.child(node_explorer_ui(
                    self.file_system.lock().as_mut().unwrap(),
                    window,
                ))
            })
            .child(
                div()
                    .w(relative(0.75))
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(self.top_bar(cx))
                    .child(div().flex_1())
                    .child(status_bar()),
            )
    }
}

impl Ui {
    pub fn new(file_system: Arc<Mutex<FileSystem>>) {
        Application::new().with_assets(Assets).run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Maximized(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| Ui {
                        active_tab: TopBarTab::Editor,
                        file_system,
                    })
                },
            )
            .unwrap();
        });
    }

    fn top_bar(&self, cx: &Context<Ui>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(32.))
            .bg(rgb(THEME.foreground))
            .flex()
            .flex_row()
            .children([
                self.top_bar_tab("Editor", Icon::Cube, TopBarTab::Editor, cx),
                self.top_bar_tab("Visualizer", Icon::Eye, TopBarTab::Visualizer, cx),
            ])
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap(px(20.))
                    .px(px(12.))
                    .child(icon_group([
                        icon(Icon::Save),
                        icon(Icon::Undo),
                        icon(Icon::Redo),
                    ]))
                    .child(icon_group([
                        icon(Icon::Move),
                        icon(Icon::Rotate),
                        icon(Icon::Scale),
                    ]))
                    .child(icon_group([icon(Icon::Grid), icon(Icon::Settings)])),
            )
    }

    fn top_bar_tab(
        &self,
        name: &'static str,
        tab_icon: Icon,
        tab: TopBarTab,
        cx: &Context<Ui>,
    ) -> impl IntoElement {
        let active = self.active_tab == tab;

        div()
            .id(name)
            .flex()
            .h(px(32.))
            .px(px(8.))
            .bg(rgb(if active {
                THEME.selection
            } else {
                THEME.foreground
            }))
            .text_color(rgb(if active { THEME.text_c1 } else { THEME.text_c2 }))
            .text_size(px(12.))
            .border_r_1()
            .border_color(rgb(THEME.border))
            .items_center()
            .justify_center()
            .gap(px(6.))
            .child(icon(tab_icon))
            .child(name)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.active_tab = tab;
                cx.notify();
            }))
    }
}

fn icon_group<const N: usize>(icons: [impl IntoElement; N]) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.))
        .children(icons)
}

fn status_bar() -> impl IntoElement {
    div()
        .w_full()
        .h(px(24.))
        .flex()
        .flex_row()
        .items_center()
        .justify_end()
        .gap(px(8.))
        .px(px(12.))
        .bg(rgb(THEME.foreground))
        .border_t_1()
        .border_color(rgb(THEME.border))
        .children([icon(Icon::Info), icon(Icon::Warning), icon(Icon::Error)])
}
