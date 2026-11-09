use std::{path::PathBuf, rc::Rc, str::FromStr};

use gpui_component_story::Open;
use gpui_kit::component::{
    ActiveTheme, WindowExt,
    highlighter::{Diagnostic, DiagnosticSeverity, Language},
    input::{self, Editor, EditorState, Input, InputEvent, InputState, Position, RopeExt, TabSize},
    v_flex,
};
use gpui_kit::*;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};

use lang::Lang;
use lsp::LspStore;
use text_convertor::TextConvertor;

mod lang;
mod lsp;
mod status;
mod text_convertor;

pub use lang::init;

pub struct CodeEditor {
    editor: Entity<EditorState>,
    go_to_line_state: Entity<InputState>,
    language: Lang,
    line_number: bool,
    indent_guides: bool,
    soft_wrap: bool,
    show_whitespaces: bool,
    folding: bool,
    readonly: bool,
    scroll_beyond_last_line: Option<usize>,
    cursor_surrounding_lines: Option<usize>,
    lsp_store: LspStore,
    _subscriptions: Vec<Subscription>,
    _lint_task: Task<()>,
}

impl CodeEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let default_language = Lang::BuiltIn(Language::Rust);
        let lsp_store = LspStore::new();

        let editor = cx.new(|cx| {
            EditorState::new(window, cx)
                .language(default_language.name().to_string())
                .line_number(true)
                .indent_guides(true)
                .tab_size(TabSize {
                    tab_size: 4,
                    hard_tabs: false,
                })
                .soft_wrap(false)
                .default_value(include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/fixtures/test.rs"
                )))
                .placeholder("Enter your code here...")
        });

        editor.update(cx, |state, cx| {
            let lsp_store = Rc::new(lsp_store.clone());
            state.lsp_mut().completion_provider = Some(lsp_store.clone());
            state.lsp_mut().code_action_providers = vec![lsp_store.clone(), Rc::new(TextConvertor)];
            state.lsp_mut().hover_provider = Some(lsp_store.clone());
            state.lsp_mut().definition_provider = Some(lsp_store.clone());
            state.lsp_mut().document_color_provider = Some(lsp_store);
            cx.notify();
        });

        let focus_handle = editor.focus_handle(cx);
        window.defer(cx, move |window, cx| {
            focus_handle.focus(window, cx);
        });

        let go_to_line_state = cx.new(|cx| InputState::new(window, cx));

        let _subscriptions = vec![cx.subscribe(&editor, |this, _editor, _: &InputEvent, cx| {
            this.lint_document(cx);
        })];

        Self {
            editor,
            go_to_line_state,
            language: default_language,
            line_number: true,
            indent_guides: true,
            soft_wrap: false,
            show_whitespaces: false,
            folding: true,
            readonly: false,
            scroll_beyond_last_line: None,
            cursor_surrounding_lines: None,
            lsp_store,
            _subscriptions,
            _lint_task: Task::ready(()),
        }
    }

    fn go_to_line(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let editor = self.editor.clone();
        let input_state = self.go_to_line_state.clone();

        window.open_alert_dialog(cx, move |dialog, window, cx| {
            input_state.update(cx, |state, cx| {
                let cursor_pos = editor.read(cx).cursor_position();
                state.set_placeholder(
                    format!("{}:{}", cursor_pos.line, cursor_pos.character),
                    window,
                    cx,
                );
                state.focus(window, cx);
            });

            dialog
                .title("Go to line")
                .child(Input::new(&input_state))
                .on_ok({
                    let editor = editor.clone();
                    let input_state = input_state.clone();
                    move |_, window, cx| {
                        let query = input_state.read(cx).value();
                        let mut parts = query
                            .split(':')
                            .map(|s| s.trim().parse::<usize>().ok())
                            .collect::<Vec<_>>()
                            .into_iter();
                        let Some(line) = parts.next().and_then(|l| l) else {
                            return false;
                        };
                        let column = parts.next().and_then(|c| c).unwrap_or(1);
                        let position = input::Position::new(
                            line.saturating_sub(1) as u32,
                            column.saturating_sub(1) as u32,
                        );

                        editor.update(cx, |state, cx| {
                            state.set_cursor_position(position, window, cx);
                        });

                        true
                    }
                })
        });
    }

    fn lint_document(&mut self, cx: &mut Context<Self>) {
        let language = self.language.name().to_string();
        let lsp_store = self.lsp_store.clone();
        let text = self.editor.read(cx).text().clone();

        self._lint_task = cx.background_spawn(async move {
            let value = text.to_string();
            let result = autocorrect::lint_for(value.as_str(), &language);

            let mut code_actions = vec![];
            let mut diagnostics = vec![];

            for item in result.lines.iter() {
                let severity = match item.severity {
                    autocorrect::Severity::Error => DiagnosticSeverity::Warning,
                    autocorrect::Severity::Warning => DiagnosticSeverity::Hint,
                    autocorrect::Severity::Pass => DiagnosticSeverity::Info,
                };

                let line = item.line.saturating_sub(1);
                let col = item.col.saturating_sub(1);

                let start = Position::new(line as u32, col as u32);
                let end = Position::new(line as u32, (col + item.old.chars().count()) as u32);
                let message = format!("AutoCorrect: {}", item.new);
                diagnostics.push(Diagnostic::new(start..end, message).with_severity(severity));

                let range = text.position_to_offset(&start)..text.position_to_offset(&end);

                let text_edit = TextEdit {
                    range: lsp_types::Range { start, end },
                    new_text: item.new.clone(),
                };

                let edit = WorkspaceEdit {
                    changes: Some(
                        std::iter::once((
                            lsp_types::Uri::from_str("file://example").unwrap(),
                            vec![text_edit],
                        ))
                        .collect(),
                    ),
                    ..Default::default()
                };

                code_actions.push((
                    range,
                    CodeAction {
                        title: format!("Change to '{}'", item.new),
                        kind: Some(CodeActionKind::QUICKFIX),
                        edit: Some(edit),
                        ..Default::default()
                    },
                ));
            }

            lsp_store.update_code_actions(code_actions.clone());
            lsp_store.update_diagnostics(diagnostics.clone());
        });
    }

    fn on_action_open(&mut self, _: &Open, window: &mut Window, cx: &mut Context<Self>) {
        let path = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: true,
            multiple: false,
            prompt: Some("Select a source file".into()),
        });

        let view = cx.entity();
        cx.spawn_in(window, async move |_, window| {
            let path = path.await.ok()?.ok()??.iter().next()?.clone();

            window
                .update(|window, cx| Self::open_file(view, path, window, cx))
                .ok()
        })
        .detach();
    }

    fn open_file(
        view: Entity<Self>,
        path: PathBuf,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        let language = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default();
        let language = Lang::from_str(&language);
        let content = std::fs::read_to_string(&path)?;

        window
            .spawn(cx, async move |window| {
                _ = view.update_in(window, |this, window, cx| {
                    _ = this.editor.update(cx, |this, cx| {
                        this.set_highlighter(language.name().to_string(), cx);
                        this.set_value(content, window, cx);
                    });

                    this.language = language;
                    cx.notify();
                });
            })
            .detach();

        Ok(())
    }
}

impl Render for CodeEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.lsp_store.is_dirty() {
            let diagnostics = self.lsp_store.diagnostics();
            self.editor.update(cx, |state, cx| {
                if let Some(set) = state.diagnostics_mut() {
                    set.clear();
                    set.extend(diagnostics);
                }
                cx.notify();
            });
        }

        v_flex()
            .id("code-editor")
            .size_full()
            .on_action(cx.listener(Self::on_action_open))
            .child(
                Editor::new(&self.editor)
                    .readonly(self.readonly)
                    .bordered(false)
                    .p_0()
                    .h(relative(1.))
                    .font_family(cx.theme().mono_font_family.clone())
                    .text_size(cx.theme().mono_font_size),
            )
    }
}
