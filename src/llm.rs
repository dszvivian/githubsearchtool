pub mod llm{


    use reqwest::{header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT}, Client};
    use serde_json::{json,Value};
    use dotenv::dotenv;



    //Using Groq Api for now 
    //For experimentation

    pub struct LLM{
        api_key: String,
        request_url: String,
        client: Client
    }


    impl LLM{

        pub fn new(request_url:String)-> Self{

            dotenv().ok().expect("Can't read.env file");  //loads the variables from .env file     
            
            let groq_api_key =  std::env::var("GROQ_API_KEY").expect("Add github access token in .env file");
       
            Self { 
                api_key: groq_api_key,
                request_url: request_url,
                client: Client::new()
             }
        }



        pub async fn generate_content(&mut self,prompt:String) -> String{

    
            let user_prompt = r#"
            {"messages": [{"role": "user", "content": "Explain the importance of fast language models"}],
             "model": "llama3-8b-8192"
             }"#;

            let mut payload:Value =  serde_json::from_str(&user_prompt).unwrap();

            payload["messages"][0]["content"] = Value::String(prompt);

            let response = self.client
                    .post(&self.request_url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}",self.api_key))
                    .json(&payload)
                    .send()
                    .await
                    .unwrap();

            let data =  response.text().await.unwrap();

            let files_list:Value = serde_json::from_str(&data).unwrap();

            files_list["choices"][0]["message"]["content"].as_str().unwrap().to_string()
        }

    }


}