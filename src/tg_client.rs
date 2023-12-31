use serde::Deserialize;
use serde_json::json;
use crate::structs::tg;

static TG_BASE_URL: &str = "https://api.telegram.org/bot";

pub struct TGClient {
    token: String,
    pub last_update_id: u64,
}

#[derive(Deserialize, Debug)]
struct UpdateResponse {
    ok: bool,
    result: Option<Vec<tg::Update>>
}

pub fn escape(s: &str) -> String {
    s.replace('_', r"\_")
        .replace('*', r"\*")
        .replace('[', r"\[")
        .replace(']', r"\]")
        .replace('(', r"\(")
        .replace(')', r"\)")
        .replace('~', r"\~")
        //.replace('`', r"\`")
        .replace('>', r"\>")
        .replace('#', r"\#")
        .replace('+', r"\+")
        .replace('-', r"\-")
        .replace('=', r"\=")
        .replace('|', r"\|")
        .replace('{', r"\{")
        .replace('}', r"\}")
        .replace('.', r"\.")
        .replace('!', r"\!")
}

impl TGClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            last_update_id: 0
        }
    }

    pub async fn updates(&self) -> Vec<tg::Update> {
        let resp = reqwest::get(format!("{}{}/getUpdates?offset={}", TG_BASE_URL, &self.token, &self.last_update_id))
            .await.unwrap()
            .json::<UpdateResponse>().await.unwrap();

        if let Some(updates) = resp.result {
            return updates;
        }

        return vec![];
    }

    pub fn set_last_update(&mut self, last_update_id: u64) {
        self.last_update_id = last_update_id;
    }


    pub async fn send_message(&self, chat_id: u64, message: &str) {
        let client = reqwest::Client::new();
        let message = escape(message);
        client.post(format!("{}{}/sendMessage", TG_BASE_URL, &self.token))
            .json(&json!(
                {
                    "chat_id": chat_id,
                    "text": message,
                    "parse_mode": "MarkdownV2"
                }
            ))
            .send()
            .await.unwrap();
    }
}
