use gpui::{IntoElement, Styled, px, rgb, svg};

use crate::ui::components::colors::THEME;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    Play,
    Stop,
    Reload,
    Add,
    Search,
    Help,
    More,
    Menu,
}

impl Icon {
    pub fn path(self) -> &'static str {
        match self {
            Icon::Play => "icons/play.svg",
            Icon::Stop => "icons/stop.svg",
            Icon::Reload => "icons/reload.svg",
            Icon::Add => "icons/add.svg",
            Icon::Search => "icons/search.svg",
            Icon::Help => "icons/help.svg",
            Icon::More => "icons/more.svg",
            Icon::Menu => "icons/menu.svg",
        }
    }

    pub fn color(self) -> u32 {
        match self {
            Icon::Play => THEME.success,
            Icon::Stop => THEME.error,
            _ => THEME.text_c2,
        }
    }
}

pub fn icon(i: Icon) -> impl IntoElement {
    icon_ex(i, 16., i.color())
}

pub fn icon_ex(i: Icon, size: f32, color: u32) -> impl IntoElement {
    svg()
        .path(i.path())
        .size(px(size))
        .flex_none()
        .text_color(rgb(color))
}
