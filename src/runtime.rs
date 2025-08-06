use deno_core::JsRuntime;
use deno_core::RuntimeOptions; 
use deno_core::error::JsError;

/// Creates a new JavaScript runtime instance with the specified options.
/// 
/// # Arguments
/// 
/// * `runtime_options` - Configuration options for initializing the JavaScript runtime.
/// 
/// # Returns
/// 
/// A mutable instance of `JsRuntime` ready for executing JavaScript code.
/// 
fn run_script(script: &str, runtime_options: RuntimeOptions) -> Result<(), Box<JsError>> {
    let mut runtime = JsRuntime::new(runtime_options);
    runtime.execute_script("workflow.js", script.to_string())?;

    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_run_script() { 
        let script = "1 + 1;";
        let runtime_options = RuntimeOptions::default();
        let result = run_script(script, runtime_options);
        assert!(result.is_ok(), "Script should run successfully");
    }
}