use std::borrow::Cow;

use gpui::{AssetSource, IntoElement, SharedString, Styled, px, rgb, svg};
use rust_embed::RustEmbed;

use crate::ui::components::colors::THEME;

#[derive(RustEmbed)]
#[folder = "src/ui/icons/icons/"]
#[prefix = "icons/"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::get(path).map(|file| file.data))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|p| p.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Icon {
    Check,
    Minus,
    ChevronRight,
    ChevronDown,
    Close,
    Grip,
    Eye,
    EyeOff,
    Lock,
    Unlock,
    Play,
    Pause,
    Step,
    Stop,
    Reload,
    Move,
    Rotate,
    Scale,
    Cube,
    Camera,
    Light,
    Grid,
    Folder,
    File,
    Save,
    Copy,
    Trash,
    Undo,
    Redo,
    Filter,
    Search,
    Add,
    Settings,
    Help,
    More,
    Menu,
    Info,
    Warning,
    Error,
}

impl Icon {
    pub fn path(self) -> &'static str {
        match self {
            Icon::Check => "icons/check.svg",
            Icon::Minus => "icons/minus.svg",
            Icon::ChevronRight => "icons/chevron_right.svg",
            Icon::ChevronDown => "icons/chevron_down.svg",
            Icon::Close => "icons/close.svg",
            Icon::Grip => "icons/grip.svg",
            Icon::Eye => "icons/eye.svg",
            Icon::EyeOff => "icons/eye_off.svg",
            Icon::Lock => "icons/lock.svg",
            Icon::Unlock => "icons/unlock.svg",
            Icon::Play => "icons/play.svg",
            Icon::Pause => "icons/pause.svg",
            Icon::Step => "icons/step.svg",
            Icon::Stop => "icons/stop.svg",
            Icon::Reload => "icons/reload.svg",
            Icon::Move => "icons/move.svg",
            Icon::Rotate => "icons/rotate.svg",
            Icon::Scale => "icons/scale.svg",
            Icon::Cube => "icons/cube.svg",
            Icon::Camera => "icons/camera.svg",
            Icon::Light => "icons/light.svg",
            Icon::Grid => "icons/grid.svg",
            Icon::Folder => "icons/folder.svg",
            Icon::File => "icons/file.svg",
            Icon::Save => "icons/save.svg",
            Icon::Copy => "icons/copy.svg",
            Icon::Trash => "icons/trash.svg",
            Icon::Undo => "icons/undo.svg",
            Icon::Redo => "icons/redo.svg",
            Icon::Filter => "icons/filter.svg",
            Icon::Search => "icons/search.svg",
            Icon::Add => "icons/add.svg",
            Icon::Settings => "icons/settings.svg",
            Icon::Help => "icons/help.svg",
            Icon::More => "icons/more.svg",
            Icon::Menu => "icons/menu.svg",
            Icon::Info => "icons/info.svg",
            Icon::Warning => "icons/warning.svg",
            Icon::Error => "icons/error.svg",
        }
    }

    pub fn color(self) -> u32 {
        match self {
            Icon::Play => THEME.success,
            Icon::Stop | Icon::Error => THEME.error,
            Icon::Warning => THEME.warning,
            Icon::Info => THEME.info,
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
