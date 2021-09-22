
pub mod parser{
    use serde_json::{Value};

    pub trait MessageParser {
        fn get_json_content(&self) -> &Vec<Value>;
        fn get_message(&self) -> String;
        fn parse_message(&self) -> Vec<String>;
    }
    
    pub struct TelegramParser {
        pub message: Value,
    }
    
    impl MessageParser for TelegramParser {
        fn get_json_content(&self)-> &Vec<Value>{
           let content = &self.message[0]["def"][0]["sseq"][0][0][1]["dt"][1][1];
           if content.is_array(){
               return content.as_array().unwrap();
           }else{
               return &self.message[1]["shortdef"].as_array().unwrap(); 
           }
        }
        fn get_message(&self) -> String {
            let result: Vec<String> = self.parse_message();
            let message = result.join("\n");
            message
        }
        fn parse_message(&self) -> Vec<String> {
            let mut result: Vec<String> = vec![];
            for value in  self.get_json_content(){
                let spanish_traduction = value.get("tr").unwrap();
                let english_traduction = value.get("t").unwrap();
    
                result.push(String::from(format!("English: {}\nSpanish: {}\n", &english_traduction, &spanish_traduction )));
            }
            result
        }
    }
    
}
