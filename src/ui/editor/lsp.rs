use std::{
    ops::Range,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Duration,
};

use gpui_kit::component::{
    highlighter::Diagnostic,
    input::{
        CodeActionProvider, CompletionProvider, DefinitionProvider, DocumentColorProvider,
        EditorState, HoverProvider, Rope, RopeExt,
    },
};
use gpui_kit::*;
use lsp_types::{
    CodeAction, CompletionContext, CompletionItem, CompletionResponse, CompletionTextEdit,
    InlineCompletionContext, InlineCompletionItem, InlineCompletionResponse, InsertReplaceEdit,
    InsertTextFormat,
};

#[derive(Clone)]
pub struct LspStore {
    completions: Arc<Vec<CompletionItem>>,
    code_actions: Arc<RwLock<Vec<(Range<usize>, CodeAction)>>>,
    diagnostics: Arc<RwLock<Vec<Diagnostic>>>,
    dirty: Arc<RwLock<bool>>,
}

impl LspStore {
    pub fn new() -> Self {
        let completions = serde_json::from_slice::<Vec<CompletionItem>>(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/completion_items.json"
        )))
        .unwrap();

        Self {
            completions: Arc::new(completions),
            code_actions: Arc::new(RwLock::new(vec![])),
            diagnostics: Arc::new(RwLock::new(vec![])),
            dirty: Arc::new(RwLock::new(false)),
        }
    }

    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        let guard = self.diagnostics.read().unwrap();
        guard.clone()
    }

    pub fn update_diagnostics(&self, diagnostics: Vec<Diagnostic>) {
        let mut guard = self.diagnostics.write().unwrap();
        *guard = diagnostics;
        *self.dirty.write().unwrap() = true;
    }

    fn code_actions(&self) -> Vec<(Range<usize>, CodeAction)> {
        let guard = self.code_actions.read().unwrap();
        guard.clone()
    }

    pub fn update_code_actions(&self, code_actions: Vec<(Range<usize>, CodeAction)>) {
        let mut guard = self.code_actions.write().unwrap();
        *guard = code_actions;
        *self.dirty.write().unwrap() = true;
    }

    pub fn is_dirty(&self) -> bool {
        let guard = self.dirty.read().unwrap();
        *guard
    }
}

fn completion_item(
    replace_range: &lsp_types::Range,
    label: &str,
    replace_text: &str,
    documentation: &str,
) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(lsp_types::CompletionItemKind::FUNCTION),
        text_edit: Some(CompletionTextEdit::InsertAndReplace(InsertReplaceEdit {
            new_text: replace_text.to_string(),
            insert: *replace_range,
            replace: *replace_range,
        })),
        documentation: Some(lsp_types::Documentation::String(documentation.to_string())),
        insert_text: None,
        ..Default::default()
    }
}

impl CompletionProvider for LspStore {
    fn completions(
        &self,
        rope: &Rope,
        offset: usize,
        trigger: CompletionContext,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<CompletionResponse>> {
        let trigger_character = trigger.trigger_character.unwrap_or_default();
        if trigger_character.is_empty() {
            return Task::ready(Ok(CompletionResponse::Array(vec![])));
        }

        let rope = rope.clone();
        let items = self.completions.clone();
        cx.background_spawn(async move {
            smol::Timer::after(Duration::from_millis(20)).await;

            if trigger_character.starts_with("/") {
                let start = offset.saturating_sub(trigger_character.len());
                let start_pos = rope.offset_to_position(start);
                let end_pos = rope.offset_to_position(offset);
                let replace_range = lsp_types::Range::new(start_pos, end_pos);

                let items = vec![
                    completion_item(
                        &replace_range,
                        "/date",
                        format!("{}", chrono::Local::now().date_naive()).as_str(),
                        "Insert current date",
                    ),
                    completion_item(&replace_range, "/thanks", "Thank you!", "Insert Thank you!"),
                    completion_item(&replace_range, "/+1", "👍", "Insert 👍"),
                    completion_item(&replace_range, "/-1", "👎", "Insert 👎"),
                    completion_item(&replace_range, "/smile", "😊", "Insert 😊"),
                    completion_item(&replace_range, "/sad", "😢", "Insert 😢"),
                    completion_item(&replace_range, "/launch", "🚀", "Insert 🚀"),
                ];
                return Ok(CompletionResponse::Array(items));
            }

            let items = items
                .iter()
                .filter(|item| item.label.starts_with(&trigger_character))
                .take(10)
                .map(|item| {
                    let mut item = item.clone();
                    item.insert_text = Some(item.label.replace(&trigger_character, ""));
                    item
                })
                .collect::<Vec<_>>();

            Ok(CompletionResponse::Array(items))
        })
    }

    fn inline_completion(
        &self,
        rope: &Rope,
        offset: usize,
        _trigger: InlineCompletionContext,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<InlineCompletionResponse>> {
        let rope = rope.clone();
        cx.background_spawn(async move {
            let point = rope.offset_to_point(offset);
            let line_start = rope.line_start_offset(point.row);
            let current_line = rope.slice(line_start..offset).to_string();

            let suggestion =
                if current_line.trim_start().starts_with("fn ") && !current_line.contains('{') {
                    Some("() {\n    // Write your code here..\n}".into())
                } else {
                    None
                };

            if let Some(insert_text) = suggestion {
                Ok(InlineCompletionResponse::Array(vec![
                    InlineCompletionItem {
                        insert_text,
                        filter_text: None,
                        range: None,
                        command: None,
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                    },
                ]))
            } else {
                Ok(InlineCompletionResponse::Array(vec![]))
            }
        })
    }

    fn is_completion_trigger(&self, _offset: usize, _new_text: &str, _cx: &mut App) -> bool {
        true
    }
}

impl CodeActionProvider for LspStore {
    fn id(&self) -> SharedString {
        "LspStore".into()
    }

    fn code_actions(
        &self,
        _state: Entity<EditorState>,
        range: Range<usize>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<CodeAction>>> {
        let mut actions = vec![];
        for (node_range, code_action) in self.code_actions().iter() {
            if !(range.start >= node_range.start && range.end <= node_range.end) {
                continue;
            }

            actions.push(code_action.clone());
        }

        Task::ready(Ok(actions))
    }

    fn perform_code_action(
        &self,
        state: Entity<EditorState>,
        action: CodeAction,
        _push_to_history: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(edit) = action.edit else {
            return Task::ready(Ok(()));
        };

        let Some((_, text_edits)) = if let Some(changes) = edit.changes {
            changes
        } else {
            return Task::ready(Ok(()));
        }
        .into_iter()
        .next() else {
            return Task::ready(Ok(()));
        };

        let state = state.downgrade();
        window.spawn(cx, async move |cx| {
            state.update_in(cx, |state, window, cx| {
                state.apply_lsp_edits(&text_edits, window, cx);
            })
        })
    }
}

impl HoverProvider for LspStore {
    fn hover(
        &self,
        text: &Rope,
        offset: usize,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Option<lsp_types::Hover>>> {
        let word = text.word_at(offset);
        if word.is_empty() {
            return Task::ready(Ok(None));
        }

        let Some(item) = self.completions.iter().find(|item| item.label == word) else {
            return Task::ready(Ok(None));
        };

        let contents = if let Some(doc) = &item.documentation {
            match doc {
                lsp_types::Documentation::String(s) => s.clone(),
                lsp_types::Documentation::MarkupContent(mc) => mc.value.clone(),
            }
        } else {
            "No documentation available.".to_string()
        };

        let hover = lsp_types::Hover {
            contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(contents)),
            range: None,
        };

        Task::ready(Ok(Some(hover)))
    }
}

const RUST_DOC_URLS: &[(&str, &str)] = &[
    ("String", "string/struct.String"),
    ("Debug", "fmt/trait.Debug"),
    ("Clone", "clone/trait.Clone"),
    ("Option", "option/enum.Option"),
    ("Result", "result/enum.Result"),
    ("Vec", "vec/struct.Vec"),
    ("HashMap", "collections/hash_map/struct.HashMap"),
    ("HashSet", "collections/hash_set/struct.HashSet"),
    ("Arc", "sync/struct.Arc"),
    ("RwLock", "sync/struct.RwLock"),
    ("Duration", "time/struct.Duration"),
];

impl DefinitionProvider for LspStore {
    fn definitions(
        &self,
        text: &Rope,
        offset: usize,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<lsp_types::LocationLink>>> {
        let Some(word_range) = text.word_range(offset) else {
            return Task::ready(Ok(vec![]));
        };
        let word = text.slice(word_range.clone()).to_string();

        let document_uri = lsp_types::Uri::from_str("file://example").unwrap();
        let start = text.offset_to_position(word_range.start);
        let end = text.offset_to_position(word_range.end);
        let symbol_range = lsp_types::Range { start, end };

        if word == "Duration" {
            let target_range = lsp_types::Range {
                start: lsp_types::Position {
                    line: 2,
                    character: 4,
                },
                end: lsp_types::Position {
                    line: 2,
                    character: 23,
                },
            };
            return Task::ready(Ok(vec![lsp_types::LocationLink {
                target_uri: document_uri,
                target_range: target_range,
                target_selection_range: target_range,
                origin_selection_range: Some(symbol_range),
            }]));
        }

        let names = RUST_DOC_URLS
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>();
        for (ix, t) in names.iter().enumerate() {
            if *t == word {
                let url = RUST_DOC_URLS[ix].1;
                let location = lsp_types::LocationLink {
                    target_uri: lsp_types::Uri::from_str(&format!(
                        "https://doc.rust-lang.org/std/{}.html",
                        url
                    ))
                    .unwrap(),
                    target_selection_range: lsp_types::Range::default(),
                    target_range: lsp_types::Range::default(),
                    origin_selection_range: Some(symbol_range),
                };

                return Task::ready(Ok(vec![location]));
            }
        }

        Task::ready(Ok(vec![]))
    }
}

impl DocumentColorProvider for LspStore {
    fn document_colors(
        &self,
        text: &Rope,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<gpui_kit::Result<Vec<lsp_types::ColorInformation>>> {
        let nodes = color_lsp::parse(&text.to_string());
        let colors = nodes
            .into_iter()
            .map(|node| {
                let start = lsp_types::Position::new(node.position.line, node.position.character);
                let end = lsp_types::Position::new(
                    node.position.line,
                    node.position.character + node.matched.chars().count() as u32,
                );

                lsp_types::ColorInformation {
                    range: lsp_types::Range { start, end },
                    color: lsp_types::Color {
                        red: node.color.r,
                        green: node.color.g,
                        blue: node.color.b,
                        alpha: node.color.a,
                    },
                }
            })
            .collect::<Vec<_>>();

        Task::ready(Ok(colors))
    }
}
