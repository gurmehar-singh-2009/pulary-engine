#![feature(default_field_values)]

use crate::scripting::ScriptingContext;
use crate::scripting::file_system::FileSystem;
use crate::ui::Ui;

mod scripting;
mod ui;

struct Engine {
    pub ui: (),
    pub scripting: ScriptingContext,
    pub file_system: FileSystem<'static>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            ui: Ui::new(),
            scripting: ScriptingContext::new(),
            file_system: FileSystem::new(),
        }
    }
}

fn main() {
    let _ = Engine::new();
}
