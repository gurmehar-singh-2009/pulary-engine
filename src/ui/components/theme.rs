use std::sync::Arc;

use gpui::{App, Hsla, rgb};
use gpui_kit::component::{Theme, ThemeMode};

use crate::ui::components::colors::THEME;

fn color(value: u32) -> Hsla {
    rgb(value).into()
}

pub fn apply(cx: &mut App) {
    let background = color(THEME.background);
    let panel = color(THEME.foreground);
    let raised = color(THEME.surface_raised);
    let border = color(THEME.border);
    let grid = color(THEME.grid);
    let text = color(THEME.text_c1);
    let text_muted = color(THEME.text_c2);
    let text_subtle = color(THEME.text_c3);
    let accent = color(THEME.accent);
    let accent_hover = color(THEME.accent_hover);
    let selection = color(THEME.selection);
    let success = color(THEME.success);
    let warning = color(THEME.warning);
    let error = color(THEME.error);
    let info = color(THEME.info);

    let mode = if background.l < 0.5 {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    };
    Theme::change(mode, None, cx);

    Theme::update(cx, |theme| {
        let colors = &mut theme.colors;

        colors.background = background;
        colors.foreground = text;
        colors.border = border;
        colors.input = border;
        colors.ring = accent;
        colors.caret = text;
        colors.selection = selection;
        colors.window_border = border;
        colors.drag_border = accent;
        colors.drop_target = selection;
        colors.chart_grid = grid;

        colors.accent = raised;
        colors.accent_foreground = text;
        colors.muted = raised;
        colors.muted_foreground = text_subtle;

        colors.primary = accent;
        colors.primary_hover = accent_hover;
        colors.primary_active = accent_hover;
        colors.primary_foreground = text;

        colors.secondary = raised;
        colors.secondary_hover = border;
        colors.secondary_active = border;
        colors.secondary_foreground = text;

        colors.button = raised;
        colors.button_hover = border;
        colors.button_active = border;
        colors.button_foreground = text;
        colors.button_primary = accent;
        colors.button_primary_hover = accent_hover;
        colors.button_primary_active = accent_hover;
        colors.button_primary_foreground = text;

        colors.popover = raised;
        colors.popover_foreground = text;

        colors.sidebar = panel;
        colors.sidebar_foreground = text;
        colors.sidebar_border = border;
        colors.sidebar_accent = raised;
        colors.sidebar_accent_foreground = text;
        colors.sidebar_primary = accent;
        colors.sidebar_primary_foreground = text;

        colors.title_bar = panel;
        colors.title_bar_border = border;
        colors.status_bar = panel;
        colors.status_bar_border = border;

        colors.tab_bar = panel;
        colors.tab = panel;
        colors.tab_foreground = text_muted;
        colors.tab_active = selection;
        colors.tab_active_foreground = text;

        colors.list = panel;
        colors.list_even = panel;
        colors.list_head = panel;
        colors.list_hover = raised;
        colors.list_active = selection;
        colors.list_active_border = accent;

        colors.table = background;
        colors.table_even = panel;
        colors.table_head = panel;
        colors.table_head_foreground = text_muted;
        colors.table_hover = raised;
        colors.table_active = selection;
        colors.table_active_border = accent;
        colors.table_row_border = border;

        colors.link = accent;
        colors.link_hover = accent_hover;
        colors.link_active = accent_hover;

        colors.success = success;
        colors.success_hover = success;
        colors.success_active = success;
        colors.success_foreground = background;
        colors.warning = warning;
        colors.warning_hover = warning;
        colors.warning_active = warning;
        colors.warning_foreground = background;
        colors.danger = error;
        colors.danger_hover = error;
        colors.danger_active = error;
        colors.danger_foreground = background;
        colors.info = info;
        colors.info_hover = info;
        colors.info_active = info;
        colors.info_foreground = background;

        colors.red = color(THEME.axis_x);
        colors.green = color(THEME.axis_y);
        colors.blue = color(THEME.axis_z);

        let mut highlight = (*theme.highlight_theme).clone();
        highlight.style.editor_background = Some(background);
        highlight.style.editor_foreground = Some(text);
        highlight.style.editor_active_line = Some(panel);
        highlight.style.editor_line_number = Some(text_subtle);
        highlight.style.editor_active_line_number = Some(text);
        highlight.style.editor_invisible = Some(grid);
        highlight.style.editor_gutter_background = Some(background);
        theme.highlight_theme = Arc::new(highlight);
    });
}
