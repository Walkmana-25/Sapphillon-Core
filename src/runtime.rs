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
pub fn run_script(script: &str, ext: Vec<OpDecl>) {
    let ext_vec: &'static [OpDecl] = Box::leak(ext.into_boxed_slice());
    let extension = Extension{
        name: "ext",
        ops: std::borrow::Cow::Borrowed(ext_vec),
        ..Default::default()
    };
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![extension],
        ..Default::default()
    });
    runtime.execute_script("workflow.js", script.to_string()).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension() {
        
        #[op2]
        fn test_op(#[serde] a: Vec<i32>) -> i32 {
            a.iter().sum()
        }
        
        let script = r#"
        Deno.core.print("aa\n");
        console.log(Deno.core.ops.test_op([1, 2, 3, 4, 5]));
        "#;

        let result = run_script(script, vec![test_op()]);
        println!("[test_extension] result: {result:?}");

    }
    
    #[test]
    fn test_run_script() { 
        let script = "1 + 1;";

        let mut sapphillon_runtime = SapphillonRuntime {
            runtime: Some(JsRuntime::new(Default::default())),
            runtime_options: Default::default(),
        };

        let result = run_script(script, vec![]);
    }
    #[test]
    fn test_run_script_hello() { 
        let script = "a = 1 + 1; console.log('Hello, world!');console.log(a);";

        let mut sapphillon_runtime = SapphillonRuntime {
            runtime: Some(JsRuntime::new(Default::default())),
            runtime_options: Default::default(),
        };

        // let result = run_script(script, &mut sapphillon_runtime);
        // assert!(result.is_ok(), "Script should run successfully");
    }
    
    #[test]
    fn test_system() {
       use deno_core::error::JsError;
use deno_core::{extension, op2, Extension, JsRuntime, RuntimeOptions};
use deno_core::serde_v8;
use deno_core::serde::{Deserialize, Serialize};
use std::rc::Rc;

#[op2]
fn test_op(#[serde] a: Vec<i32>) -> i32 {
    a.iter().sum()
}

// 拡張定義
extension!(
    my_ext,
    ops = [test_op],
);

    // RuntimeOptions に ops を含む拡張を登録
    let ext: Extension = my_ext::init();

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    // JSコード内で ops を呼び出す
    let script = r#"
        Deno.core.print("Running test_op...\n");
        let sum = Deno.core.ops.test_op([1, 2, 3, 4, 5]);
        Deno.core.print("Sum: " + sum + "\n");
    "#;

    runtime.execute_script("test.js", script.to_string()).unwrap();

    }
}