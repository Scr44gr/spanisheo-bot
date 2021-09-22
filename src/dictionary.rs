

pub mod dictapi {
    use reqwest::Error;
    use serde_json::Value;



    fn search(query: &str) -> Result<String, Error> {
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let url = format!(
            "https://www.dictionaryapi.com/api/v3/references/spanish/json/{}?key={}",
            query, access_token
        );
        // response
        Ok(reqwest::blocking::get(url)?.text()?)
    }

    pub fn search_word(query: &str) -> Result<Value, ()>{
        let response: String = search(query).unwrap();
        let value = serde_json::from_str(&response);

        if value.is_ok(){
            let content: Value = value.unwrap();
            Ok(content)
        }else{
            Err(())
        }
    }
}
