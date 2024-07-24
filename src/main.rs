use reqwest::{header::{HeaderMap,HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT}, Client};
// use serde::{Serialize,Deserialize};
use serde_json::Value;
use std::io;


mod githubsearchtool;
use githubsearchtool::githubsearchtool::GithubSearchTool;


#[tokio::main]
async fn main() {

    // let client = Client::new();

    let mut input = String::new();
    println!("Enter github repo:  ");
    io::stdin().read_line(&mut input).unwrap();
    let request_url = format!("https://api.github.com/repos/{input}/contents/");
   

    let mut a = GithubSearchTool::new(request_url);
    let contents =  a.get_contents(None).await;



    println!("{:#?}",contents);
}