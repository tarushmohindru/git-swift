use gemini_rs::Conversation;

pub async fn generate_commit_message(
    diff: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut convo = Conversation::new(api_key.to_string(), "gemini-1.5-flash".to_string());

    let prompt = format!(
           "Analyze the following git diff and generate a concise commit message adhering to conventional commit message standards (e.g., 50-character summary line, followed by a detailed body if necessary). Focus primarily on code logic, structure, and refactoring changes, while summarizing dependency updates briefly. Highlight:

           1. Changes to the logic and structure of the code.
           2. Any refactoring, such as new folder structures or reorganization.
           3. New features, bug fixes, or improvements in the code.
           4. Briefly mention dependency changes but avoid overemphasizing them.

           Return only the commit message text, formatted properly, without any additional context or explanations.

           Git Diff:
           {}
           ",
           diff
       );
    let response = convo.prompt(&prompt).await;
    Ok(response.trim().to_string())
}