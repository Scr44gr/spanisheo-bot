mod dictionary;

use dotenv::dotenv;
mod telegram_message_parser;
use crate::telegram_message_parser::parser::{MessageParser, TelegramParser};
use futures::stream::StreamExt;
use telegram_bot::*;

const MAX_COMMAND_SIZE: usize = 1;

#[derive(Debug, Clone)]
struct CommandError {
    message: String,
}

impl CommandError {
    fn new(message: &str) -> CommandError {
        CommandError {
            message: message.to_string(),
        }
    }
}

fn find_word(word: &str) -> String {
    let content = dictionary::dictapi::search_word(word);
    if content.is_ok() {
        let content = content.unwrap();
        let parser = TelegramParser { message: content };
        parser.get_message()
    } else {
        String::from("Word not found")
    }
}

fn get_command(text: &str) -> Result<String, CommandError> {
    let command: Vec<&str> = text.split(" ").collect::<Vec<&str>>();

    if command.len() > MAX_COMMAND_SIZE {
        match command[0] {
            "/d" => return Ok(String::from(command[1])),
            _ => return Err(CommandError::new(&"Command not found")),
        };
    }
    Err(CommandError {
        message: "Command not found".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                let text = get_command(&data);

                match text {
                    Ok(word) => {
                        let response = find_word(&word);
                        api.send(message.text_reply(response)).await?;
                    }
                    Err(error) => {
                        api.send(message.text_reply(error.message)).await?;
                    }
                }
            }
        }
    }
    Ok(())
}
