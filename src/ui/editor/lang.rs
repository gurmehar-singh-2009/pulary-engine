use gpui_kit::component::highlighter::{Language, LanguageConfig, LanguageRegistry};

pub enum Lang {
    BuiltIn(Language),
    External(&'static str),
}

impl Lang {
    pub fn name(&self) -> &str {
        match self {
            Lang::BuiltIn(lang) => lang.name(),
            Lang::External(lang) => lang,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "nv" => Lang::External("navi"),
            _ => Lang::BuiltIn(Language::from_str(s)),
        }
    }
}

pub fn init() {
    LanguageRegistry::singleton().register(
        "navi",
        &LanguageConfig::new(
            "navi",
            tree_sitter_navi::LANGUAGE.into(),
            vec![],
            tree_sitter_navi::HIGHLIGHTS_QUERY,
            "",
            "",
        ),
    );
}
