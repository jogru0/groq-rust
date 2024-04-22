pub mod groq {
    use reqwest::blocking::Client;

    pub struct Groq {
        api_key: String,
        client: Client,
    }

    impl Groq {
        pub fn new(api_key: String) -> Self {
            Self {
                api_key,
                client: Client::new(),
            }
        }

        pub fn test_request(&self) -> String {
            let response = self.client
                .post("https://api.groq.com/openai/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .body("{\"messages\": [{\"role\": \"user\", \"content\": \"Explain the importance of fast language models\"}], \"model\": \"mixtral-8x7b-32768\"}")
                .send().unwrap();

            response.text().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::groq::Groq;

    #[test]
    fn basic_test() {
        let api_key = env::var("GROQ_API_KEY").unwrap();
        let groq = Groq::new(api_key);

        println!("{}", groq.test_request())
    }
}
