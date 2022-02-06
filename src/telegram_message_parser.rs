pub mod parser {
    use serde_json::Value;

    pub trait MessageParser {
        fn get_json_content(&self) -> Vec<Value>;
        fn get_message(&self) -> String;
    }

    pub struct TelegramParser {
        pub message: Value,
    }

    impl MessageParser for TelegramParser {
        fn get_json_content(&self) -> Vec<Value> {

            let content = &self.message[0]["shortdef"];
            let mut response: Vec<Value> = Vec::new();

            if content.is_array() {
                if !content.as_array().is_none() {
                    response = content.as_array().unwrap().to_vec();
                }
            } else {
                if !self.message[1]["shortdef"].as_array().is_none() {
                    response = self.message[1]["shortdef"].as_array().unwrap().to_vec();
                }
            }
            response
        }
        fn get_message(&self) -> String {
            let result: Vec<Value> = self.get_json_content();
            let mut message: String = String::from("");
            for item in result {
                message.push_str(&item.as_str().unwrap());
                message.push_str("\n\n");
            }
            message
        }
    }
}
