//! See: <https://rune-rs.github.io/book/introduction.html>

use rune::runtime::{Args, GuardedArgs};
use rune::{Context, Diagnostics, Source, Sources, ToTypeHash, Value, Vm, VmError};

pub mod file_system;

/// The scripting engine. This will handle everything with Rune.
/// Technically though, EVERYTHING is inside Rune!
pub struct ScriptingContext {
    /// The Rune context.
    context: Context,
    /// The script sources. Currently should only be the editor.
    sources: Sources,
    /// The diagnostics report config.
    diagnostics: Diagnostics,

    /// The virtual machine itself. Only created when a script is ran.
    vm: Option<Vm>,
}

impl ScriptingContext {
    /// Create a new scripting engine instance.
    pub fn new() -> Self {
        let context = Context::new();
        let sources = Sources::new();
        let diagnostics = Diagnostics::new();

        Self {
            context,
            sources,
            diagnostics,

            vm: None,
        }
    }

    /// Run the script.
    pub fn run_script(&mut self, content: &str) {
        let editor_source = Source::memory("pub fn add(a, b) { a + b }").unwrap();
        self.sources.insert(editor_source).unwrap();

        // Run the script in the VM.
        let result = rune::prepare(&mut self.sources)
            .with_context(&self.context)
            .with_diagnostics(&mut self.diagnostics)
            .build_vm()
            .unwrap();

        self.vm = Some(result);
    }

    /// Calls a script function by the name.
    ///
    /// Returns `rune::Value::empty()` if not loaded.
    #[must_use]
    pub fn vm_call(
        &mut self,
        func: impl ToTypeHash,
        args: impl Args + GuardedArgs,
    ) -> Result<Value, VmError> {
        if let Some(vm) = &mut self.vm {
            vm.call(func, args)
        } else {
            Ok(Value::empty())
        }
    }
}
