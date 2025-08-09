#![warn(clippy::field_reassign_with_default)]

use std::borrow::Cow;

use deno_core::JsRuntime;
use deno_core::RuntimeOptions; 
use deno_core::error::JsError;
use deno_core::Extension;
use deno_core::OpDecl;
use deno_core::extension;
use deno_core::op2;
use deno_core::serde_v8;
use deno_core::serde;
use deno_core::serde_json;

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
    match sapphillon_runtime.runtime {
        Some(ref mut runtime) => {
            runtime.execute_script("workflow.js", script.to_string())?;
        }
        None => {
            let mut runtime = JsRuntime::new(Default::default());
            runtime.execute_script("workflow.js", script.to_string())?;
            sapphillon_runtime.runtime = Some(runtime);
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use deno_core::*;

    #[test]
    fn test_extension() {
        
        #[op2]
        fn test_op(#[serde] a: Vec<i32>) -> i32 {
            a.iter().sum()
        }
        
        extension!(
            runtime_js,
            ops = [test_op],
        );
        
        let mut runtime = SapphillonRuntime::new();
        runtime.runtime_options.extensions = vec![runtime_js::init()];
        runtime.runtime_options.module_loader = Some(Rc::new(FsModuleLoader));

        let script = r#"
        Deno.core.print("aa\n");
        console.log(Deno.core.ops.test_op([2, 3]));
        "#;

        let result = run_script(script, &mut runtime);
        println!("[test_extension] result: {result:?}");

    }
    
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