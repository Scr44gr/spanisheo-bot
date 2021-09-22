mod dictionary;
use dotenv::dotenv;
mod telegram_message_parser;
use crate::telegram_message_parser::parser::{TelegramParser, MessageParser};

fn main(){
    dotenv().ok();
    let query_search = String::from("Hello");
    let response = dictionary::dictapi::search_word(&query_search);
    if response.is_ok(){
        let parser = TelegramParser{message: response.unwrap()};
        print!("query: {}\n\n{}", &query_search, parser.get_message());
    }else{
        print!("Error, {} not found.", &query_search);
    }

}