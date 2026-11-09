#![feature(default_field_values)]

use std::sync::{Arc, Mutex};

use crate::scripting::ScriptingContext;
use crate::scripting::file_system::FileSystem;
use crate::ui::Ui;

mod scripting;
mod ui;

struct Engine {
    pub ui: (),
    pub scripting: ScriptingContext,
    pub file_system: Arc<Mutex<FileSystem>>,
}

impl Engine {
    pub fn new() -> Self {
        let scripting = ScriptingContext::new();
        let file_system = Arc::new(Mutex::new(FileSystem::new()));

        Self {
            ui: Ui::new(file_system.clone()),
            scripting,
            file_system,
        }
    }
}

fn main() {
    let _ = Engine::new();
}
