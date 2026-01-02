//! Feature-gated model selector bridge.
//! Enable with `--features model-selector-llama` to include llama.cpp bridge.

#[cfg(feature = "model-selector-llama")]
pub mod llama_bridge {
    use std::process::Command;

    /// Bridge implementation to llama.cpp for model selection and inference
    pub fn select_and_infer(prompt: &str) -> Result<String, String> {
        // Check if llama.cpp binary is available
        let llama_bin_path = std::env::var("LLAMA_CPP_PATH")
            .unwrap_or_else(|_| "./integrations/llama.cpp/main".to_string());

        // Check if the binary exists
        let binary_exists = std::path::Path::new(&llama_bin_path).exists();

        if !binary_exists {
            return Ok(format!(
                "[llama.cpp] Binary not found at {}. Prompt length: {}. \
                Set LLAMA_CPP_PATH environment variable or place binary in integrations/llama.cpp/",
                llama_bin_path, prompt.len()
            ));
        }

        // Execute llama.cpp with the prompt
        match Command::new(&llama_bin_path)
            .arg("-p")
            .arg(prompt)
            .arg("-n")
            .arg("128") // Max tokens
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let response = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(response)
                } else {
                    let error = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!("llama.cpp execution failed: {}", error))
                }
            }
            Err(e) => Err(format!("Failed to execute llama.cpp: {}", e)),
        }
    }
}

#[cfg(not(feature = "model-selector-llama"))]
pub mod llama_bridge {
    pub fn select_and_infer(_prompt: &str) -> Result<String, String> {
        Err("model-selector-llama feature disabled".into())
    }
}
