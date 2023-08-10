mod cli;
mod tg_client;
pub mod structs;
mod gpt_client;

use std::time::Duration;
use clap::Parser;
use tokio::time::sleep;
use crate::gpt_client::GPTClient;
use crate::structs::gpt;
use crate::tg_client::TGClient;

#[tokio::main]
async fn main() {
    let args = cli::get_arguments();
    // Open a connection to the mini-redis address.1
    let tg_token = args.get_one::<String>("tg_token").unwrap().as_str();
    let gpt_token = args.get_one::<String>("gpt_token").unwrap().as_str();
    let allowed_users: Vec<String> = args.get_one::<String>("allowed_users")
        .unwrap().split(",")
        .map(|s| s.trim().to_string()).collect() ;
    let mut tg_client = TGClient::new(tg_token.to_string());
    let gpt_client = GPTClient::new(gpt_token.to_string());

    let mut first_run = true;

    loop {
        let mut updates = tg_client.updates().await;
        if !first_run {
            for update in &updates {
                if let Some(message) = &update.message {
                    let text = &message.text.clone().unwrap_or(String::new());
                    if text.is_empty() {
                        continue;
                    }

                    let username;

                    if let Some(user) = &message.from {
                        if user.is_bot {
                            continue;
                        }

                        if let Some(name) = &user.username {
                            if !allowed_users.contains(name) {
                                continue;
                            } else {
                                username = name;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    println!("Receive message from {}: {}, wait GTP response...", username, text);

                    if let Ok(completion) = gpt_client.completion(&text).await {
                        for choice in completion.choices {
                            let send_message = choice.message.content;
                            println!("Sending message: {}", send_message);
                            tg_client.send_message(message.chat.id, &send_message, username).await;
                        }
                    }
                }
            }
        }

        first_run = false;
        if let Some(last) = &updates.last() {
            tg_client.set_last_update(last.update_id + 1);
        }
        sleep(Duration::from_secs(1)).await;
    }
}