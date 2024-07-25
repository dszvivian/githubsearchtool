use std::io;

mod githubsearchtool;
use githubsearchtool::githubsearchtool::GithubSearchTool;


mod llm;
use llm::llm::LLM;

#[tokio::main]
async fn main() {

    let mut input = String::new();
    println!("Enter github repo:  ");
    io::stdin().read_line(&mut input).unwrap();
    let request_url = format!("https://api.github.com/repos/{input}/contents/");
   

    let mut a = GithubSearchTool::new(request_url);
    let github_file_contents =  a.get_contents(None).await;

    let groq_request =  "https://api.groq.com/openai/v1/chat/completions";


    let system_prompt =  String::from("
    Create a blog post based on the given Github Contents\n
    and generate Step by Step Tutorial on creating that project.\n
    Generate only Technical Details.
    Keep it Short.\n
    ");

    let final_prompt =  augumented_prompt(github_file_contents, system_prompt);
    let mut a = LLM::new(groq_request.to_string());
    let content = a.generate_content(final_prompt).await;

    println!("{:#?}",content);
}


fn augumented_prompt(github_file_contents: Vec<(String,String)>, system_prompt: String)-> String{
    let mut files_content = String::from("List of Files: \n");

    for i in 0..github_file_contents.len(){
        files_content.push_str(format!("{}:\n {}\n\n",github_file_contents[i].0,github_file_contents[i].1).as_str());
    }

    files_content.push_str(&system_prompt);

    files_content
}