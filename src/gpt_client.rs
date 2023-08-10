use serde_json::json;
use crate::structs::gpt;

static GPT_BASE_URL: &str = "https://api.openai.com/v1";

pub struct GPTClient {
    token: String,
    model: String,
}

impl GPTClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            model: String::from("gpt-3.5-turbo"),
        }
    }

    pub async fn completion(&self, message: &str) -> Result<gpt::Completion, ()> {
        let client = reqwest::Client::new();

        let resp = client.post(format!("{}/chat/completions", GPT_BASE_URL))
            .bearer_auth(&self.token)
            .json(&json!(
                {
                    "model": &self.model,
                    "messages": [
                        {
                            "role": "user",
                            "content": message
                        }
                    ],
                }
            ))
            .send().await;

        if let Ok(result) = resp {
            let out = result.json::<gpt::Completion>().await;
            if let Ok(completion) = out {
                return Ok(completion);
            }
        }

        Err(())
    }
}
