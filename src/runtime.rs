#![warn(clippy::field_reassign_with_default)]

use std::borrow::Cow;

use deno_core::JsRuntime;
use deno_core::RuntimeOptions; 
use deno_core::error::JsError;
use deno_core::Extension;
use deno_core::OpDecl;

pub struct SapphillonRuntime {
    runtime: Option<JsRuntime>,
    runtime_options: RuntimeOptions,
}

impl SapphillonRuntime {
    pub fn new() -> Self {
        SapphillonRuntime {
            runtime: None,
            runtime_options: Default::default(),
        }
    }

    pub fn initialize_runtime(&mut self, extensions: Vec<(Vec<OpDecl>, &'static str)>) -> Result<(), Box<JsError>> {
        let mut extension_vec: Vec<Extension> = Vec::new();
        for (ops, name) in extensions {
            let ext = Extension {
                name,
                deps: &[],
                js_files: Cow::Borrowed(&[]),
                esm_files: Cow::Borrowed(&[]),
                lazy_loaded_esm_files: Cow::Borrowed(&[]),
                esm_entry_point: None,
                ops: Cow::Owned(ops),
                objects: Cow::Borrowed(&[]),
                external_references: Cow::Borrowed(&[]),
                global_template_middleware: None,
                global_object_middleware: None,
                op_state_fn: None,
                needs_lazy_init: false,
                middleware_fn: None,
                enabled: true,
            };

            extension_vec.push(ext);
        }

        if self.runtime.is_none() {
            let options = RuntimeOptions {
                extensions: extension_vec,
                ..Default::default()
            };
            self.runtime = Some(JsRuntime::new(options));
        }

        Ok(())
    }
}

impl Default for SapphillonRuntime {
    fn default() -> Self {
        SapphillonRuntime::new()
    }
}


 
/// A mutable instance of `JsRuntime` ready for executing JavaScript code.
///
/// Executes a JavaScript script within the provided SapphillonRuntime.
///
/// This function takes a JavaScript script as a string and executes it within the context of the
/// provided SapphillonRuntime instance. The script is executed with the module specifier "workflow.js".
///
/// # Arguments
///
/// * `script` - A string slice containing the JavaScript code to be executed.
/// * `sapphillon_runtime` - A mutable reference to the SapphillonRuntime instance where the script will be executed.
///
/// # Returns
///
/// * `Result<(), Box<JsError>>` - Returns Ok(()) if the script executes successfully, or an error wrapped in Box<JsError> if an error occurs.
///
fn run_script(script: &str, sapphillon_runtime: &mut SapphillonRuntime) -> Result<(), Box<JsError>> {
    sapphillon_runtime.runtime.as_mut().unwrap().execute_script("workflow.js", script.to_string())?;

    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_run_script() { 
        let script = "1 + 1;";

        let mut sapphillon_runtime = SapphillonRuntime {
            runtime: Some(JsRuntime::new(Default::default())),
            runtime_options: Default::default(),
        };

        let result = run_script(script, &mut sapphillon_runtime);
        assert!(result.is_ok(), "Script should run successfully");
    }
    #[test]
    fn test_run_script_hello() { 
        let script = "a = 1 + 1; console.log('Hello, world!');console.log(a);";

        let mut sapphillon_runtime = SapphillonRuntime {
            runtime: Some(JsRuntime::new(Default::default())),
            runtime_options: Default::default(),
        };

        let result = run_script(script, &mut sapphillon_runtime);
        assert!(result.is_ok(), "Script should run successfully");
    }
}