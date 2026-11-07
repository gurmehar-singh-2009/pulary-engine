//! See: https://rune-rs.github.io/book/introduction.html

use gpui::SharedString;
use rune::alloc::String;
use rune::runtime::{Args, GuardedArgs};
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Source, Sources, Value, Vm, VmError};

pub struct ScriptingContext {
    context: Context,
    sources: Sources,
    diagnostics: Diagnostics,

    vm: Option<Vm>,
}

impl ScriptingContext {
    pub fn new() -> Self {
        let context = Context::new();
        let mut sources = Sources::new();
        let diagnostics = Diagnostics::new();

        Self {
            context,
            sources,
            diagnostics,

            vm: None,
        }
    }

    pub fn run_script(&mut self, content: &str) {
        let editor_source = Source::memory("pub fn add(a, b) { a + b }").unwrap();
        self.sources.insert(editor_source);

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
    pub fn vm_call<T: Args + GuardedArgs>(
        &mut self,
        func: String,
        args: T,
    ) -> Result<Value, VmError> {
        if let Some(vm) = &mut self.vm {
            vm.call(&[func], args)
        } else {
            Ok(Value::empty())
        }
    }
}
