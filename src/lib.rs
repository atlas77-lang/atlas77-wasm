use wasm_bindgen::prelude::*;
use atlas_77::{build, run, CompilationFlag};


// Set panic hook for better error messages in browser
#[cfg(feature = "console_error_panic_hook")]
pub use console_error_panic_hook::set_once as set_panic_hook;

#[wasm_bindgen]
pub struct ExecutionResult {
    success: bool,
    stdout: String,
    stderr: String,
    exit_code: i32,
}

#[wasm_bindgen]
impl ExecutionResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn stdout(&self) -> String {
        self.stdout.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn stderr(&self) -> String {
        self.stderr.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
}

/// Compile and execute Atlas 77 source code
#[wasm_bindgen]
pub fn compile_and_run(source_code: String) -> Result<ExecutionResult, String> {
    // Compile the source code
    let res = match run(source_code, CompilationFlag::Debug, true) {
        Ok(_) => ExecutionResult {
            success: true,
            stdout: String::from("Execution completed successfully."),
            stderr: String::new(),
            exit_code: 0,
        },
        Err(e) => return Err(format!("{}", e)),
    };
    Ok(res)
}

/// Get the compiler version
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Format source code (future feature)
#[wasm_bindgen]
pub fn format_code(source_code: String) -> Result<String, String> {
    // TODO: Implement code formatter
    Ok(source_code)
}

/// Validate syntax without execution
#[wasm_bindgen]
pub fn check_syntax(source_code: String) -> Result<bool, String> {
    match build(source_code, CompilationFlag::Debug, true) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("{}", e)),
    }
}