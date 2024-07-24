pub mod githubsearchtool{

    use reqwest::{header::{HeaderMap,HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT}, Client};
    use serde_json::Value;
    use dotenv::dotenv;
    

    pub struct GithubSearchTool{
        headers: HeaderMap,
        request_url: String,
        client: Client
    }


    impl GithubSearchTool{

        pub fn new(request_url:String)-> Self{

            dotenv().ok().expect("Can't read.env file");  //loads the variables from .env file     
            
            let github_access_token =  std::env::var("GITHUB_ACCESS_TOKEN").expect("Add github access token in .env file");

            let auth_value = format!("Bearer: {github_access_token}");

            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT,HeaderValue::from_static("application/vnd.github+json"));
            headers.insert(AUTHORIZATION,HeaderValue::from_str(&auth_value).unwrap());
            headers.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));
            headers.insert(USER_AGENT, HeaderValue::from_static("test app"));
        
        
            Self { 
                headers: headers,
                request_url: request_url,
                client: Client::new()
             }
        }

        pub async fn get_files(&mut self,request_url:String) -> Value{
            let response = self.client
                    .get(&request_url)
                    .headers(self.headers.clone())
                    .send()
                    .await.unwrap();
            
            let data = response.text().await.unwrap();

            serde_json::from_str(&data).unwrap()
        }


        pub async fn get_contents(&mut self,request_url:Option<String>)  -> Vec<(String, String)> {

            let files_list:Value = self.get_files(self.request_url.clone()).await;
            let len_files = files_list.as_array().unwrap().len();

            let mut contents_list:Vec<(String,String)> = Vec::new();

            

            //todo
            //Not the ideal way to get contents
            //here i am getting raw content from api
            //ideal way is to: get the base64 encoded content --. returns Json format 
            //decode 
            //So that we get additional  information like file type etc 
            for i in 1..len_files{
                let file_name = files_list.get(i).unwrap().get("name").unwrap().as_str().unwrap();

                let final_request_url = request_url.clone().unwrap_or(self.request_url.clone());
                let file_request_url = final_request_url + "/" + file_name;

                let request_contents =  self.client
                                                    .get(&file_request_url)
                                                    .headers(self.headers.clone())
                                                    .header(ACCEPT, "application/vnd.github.VERSION.raw") //get the contents in raw format
                                                    .send()
                                                    .await.unwrap();
                let data = request_contents.text().await.unwrap();


                //Check wheather its a folder

                //recursive search
                //needs a Box pointer 
                //todo: add recursive files search
                // if data.starts_with("[{"){
                //     self.get_contents(Some(format!("{file_request_url}"))).await;
                // }

                // reason we can't do this below operation in a single line is:
                // since we are  using as_str() it takes the ownership of content and drops 
                // Which creates a dangling pointer
                // So we use let binding while cloning 
                // then apply as_str() in new line

                // let v:Value = serde_json::from_str(&data).unwrap();
                // let content = v.get("content").unwrap().as_str().unwrap() ;               
                // let content_as_utf8 = STANDARD.decode(content).unwrap();
                // let content_as_str = String::from_utf8(content_as_utf8).unwrap();

                // println!("\n{:#?}\n",file_name);

                // println!("{:#?}\n\n",data);

                contents_list.push((file_name.to_string(),data));
            }


            contents_list
        }

        

    }



}