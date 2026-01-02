// SOP parser implementation
// Parse production SOP files and return structured steps.
#[allow(dead_code)]
pub fn parse_sop(text: &str) -> Vec<String> {
    // Implement real parser for SOP documents
    let mut steps = Vec::new();

    // Parse SOP text into structured steps
    // Expect format: numbered steps like "1. Step description" or "Step 1: Description"
    for line in text.lines() {
        let trimmed = line.trim();

        // Skip empty lines and headers
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Extract numbered steps
        if trimmed.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
            // Remove numbering prefix (e.g., "1. " or "1) " or "Step 1: ")
            let step_text = trimmed
                .split_once('.')
                .or_else(|| trimmed.split_once(')'))
                .or_else(|| trimmed.split_once(':'))
                .map(|(_, rest)| rest.trim())
                .unwrap_or(trimmed);

            if !step_text.is_empty() {
                steps.push(step_text.to_string());
            }
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            // Handle bullet points
            steps.push(trimmed[2..].trim().to_string());
        }
    }

    steps
}
