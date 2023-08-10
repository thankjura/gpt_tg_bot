use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CompletionChoiceMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct CompletionChoice {
    pub index: u64,
    pub message: CompletionChoiceMessage,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct CompletionUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Deserialize, Debug)]
pub struct Completion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: CompletionUsage,
}