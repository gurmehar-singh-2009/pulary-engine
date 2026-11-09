use gpui_kit::component::{
    IconName, Sizable,
    button::{Button, ButtonVariants as _},
    h_flex,
};
use gpui_kit::{prelude::FluentBuilder, *};

use super::CodeEditor;

impl CodeEditor {
    pub fn status_items(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        h_flex()
            .items_center()
            .child(self.render_line_number_button(window, cx))
            .child(self.render_soft_wrap_button(window, cx))
            .child(self.render_show_whitespaces_button(window, cx))
            .child(self.render_indent_guides_button(window, cx))
            .child(self.render_folding_button(window, cx))
            .child(self.render_readonly_button(window, cx))
            .child(self.render_scroll_beyond_last_line_button(window, cx))
            .child(self.render_cursor_surrounding_lines_button(window, cx))
            .child(self.render_go_to_line_button(window, cx))
    }

    fn render_line_number_button(&self, _: &mut Window, cx: &mut Context<Self>) -> Button {
        Button::new("line-number")
            .when(self.line_number, |this| this.icon(IconName::Check))
            .label("Line Number")
            .ghost()
            .xsmall()
            .on_click(cx.listener(|this, _, window, cx| {
                this.line_number = !this.line_number;
                this.editor.update(cx, |state, cx| {
                    state.set_line_number(this.line_number, window, cx);
                });
                cx.notify();
            }))
    }

    fn render_soft_wrap_button(&self, _: &mut Window, cx: &mut Context<Self>) -> Button {
        Button::new("soft-wrap")
            .ghost()
            .xsmall()
            .when(self.soft_wrap, |this| this.icon(IconName::Check))
            .label("Soft Wrap")
            .on_click(cx.listener(|this, _, window, cx| {
                this.soft_wrap = !this.soft_wrap;
                this.editor.update(cx, |state, cx| {
                    state.set_soft_wrap(this.soft_wrap, window, cx);
                });
                cx.notify();
            }))
    }

    fn render_show_whitespaces_button(&self, _: &mut Window, cx: &mut Context<Self>) -> Button {
        Button::new("show-whitespace")
            .ghost()
            .xsmall()
            .when(self.show_whitespaces, |this| this.icon(IconName::Check))
            .label("Show Whitespaces")
            .on_click(cx.listener(|this, _, window, cx| {
                this.show_whitespaces = !this.show_whitespaces;
                this.editor.update(cx, |state, cx| {
                    state.set_show_whitespaces(this.show_whitespaces, window, cx);
                });
                cx.notify();
            }))
    }

    fn render_indent_guides_button(&self, _: &mut Window, cx: &mut Context<Self>) -> Button {
        Button::new("indent-guides")
            .ghost()
            .xsmall()
            .when(self.indent_guides, |this| this.icon(IconName::Check))
            .label("Indent Guides")
            .on_click(cx.listener(|this, _, window, cx| {
                this.indent_guides = !this.indent_guides;
                this.editor.update(cx, |state, cx| {
                    state.set_indent_guides(this.indent_guides, window, cx);
                });
                cx.notify();
            }))
    }

    fn render_folding_button(&self, _: &mut Window, cx: &mut Context<Self>) -> Button {
        Button::new("folding")
            .ghost()
            .xsmall()
            .when(self.folding, |this| this.icon(IconName::Check))
            .label("Folding")
            .on_click(cx.listener(|this, _, window, cx| {
                this.folding = !this.folding;
                this.editor.update(cx, |state, cx| {
                    state.set_folding(this.folding, window, cx);
                });
                cx.notify();
            }))
    }

    fn render_readonly_button(&self, _: &mut Window, cx: &mut Context<Self>) -> Button {
        Button::new("readonly")
            .ghost()
            .xsmall()
            .when(self.readonly, |this| this.icon(IconName::Check))
            .label("Read only")
            .on_click(cx.listener(|this, _, _window, cx| {
                this.readonly = !this.readonly;
                cx.notify();
            }))
    }

    fn cycle_rows(v: Option<usize>) -> Option<usize> {
        match v {
            None => Some(0),
            Some(0) => Some(3),
            Some(3) => Some(8),
            _ => None,
        }
    }

    fn rows_label(v: Option<usize>) -> String {
        match v {
            None => "default".to_string(),
            Some(n) => n.to_string(),
        }
    }

    fn render_scroll_beyond_last_line_button(
        &self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Button {
        Button::new("scroll-beyond-last-line")
            .ghost()
            .xsmall()
            .label(format!(
                "Scroll Beyond: {}",
                Self::rows_label(self.scroll_beyond_last_line)
            ))
            .on_click(cx.listener(|this, _, window, cx| {
                this.scroll_beyond_last_line = Self::cycle_rows(this.scroll_beyond_last_line);
                this.editor.update(cx, |state, cx| {
                    state.set_scroll_beyond_last_line(this.scroll_beyond_last_line, window, cx);
                });
                cx.notify();
            }))
    }

    fn render_cursor_surrounding_lines_button(
        &self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Button {
        Button::new("cursor-surrounding-lines")
            .ghost()
            .xsmall()
            .label(format!(
                "Cursor Surrounding: {}",
                Self::rows_label(self.cursor_surrounding_lines)
            ))
            .on_click(cx.listener(|this, _, window, cx| {
                this.cursor_surrounding_lines = Self::cycle_rows(this.cursor_surrounding_lines);
                this.editor.update(cx, |state, cx| {
                    state.set_cursor_surrounding_lines(this.cursor_surrounding_lines, window, cx);
                });
                cx.notify();
            }))
    }

    fn render_go_to_line_button(&self, _: &mut Window, cx: &mut Context<Self>) -> Button {
        let position = self.editor.read(cx).cursor_position();
        let cursor = self.editor.read(cx).cursor();

        Button::new("line-column")
            .ghost()
            .xsmall()
            .label(format!(
                "{}:{} ({} byte)",
                position.line + 1,
                position.character + 1,
                cursor
            ))
            .tooltip("Go to Line/Column")
            .on_click(cx.listener(Self::go_to_line))
    }
}
