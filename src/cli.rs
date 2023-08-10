use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn get_arguments() -> ArgMatches {
    let out = Command::new("bot")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("ChatGPT telegram bot")
        .arg(
            Arg::new("gpt_token")
            .env("GPT_TOKEN")
            .short('g')
            .long("gpt-token")
            .action(ArgAction::Set)
            .value_name("GPT_TOKEN")
            .help("Chat GPT token")
            .required(true)
        )
        .arg(
            Arg::new("tg_token")
            .env("TG_TOKEN")
            .short('t')
            .long("tg-token")
            .action(ArgAction::Set)
            .value_name("TG_TOKEN")
            .help("Telegram bot token")
            .required(true)
        ).arg(
            Arg::new("allowed_users")
                .env("ALLOWED_USERS")
                .short('a')
                .long("allowed-users")
                .action(ArgAction::Set)
                .value_name("USER1,USER2")
                .help("Allowed telegram users")
                .required(true)
        );
    return out.get_matches();
}