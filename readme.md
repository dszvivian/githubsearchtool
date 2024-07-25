# GitHub Search Tool


### Final Goal:

- To be able to generate documentation/Blog Post from the repo provided
- In a effective way
- Learn about RAG ---> Implementing it from Scratch


## Installation:

- create a .env file and add

```
GITHUB_ACCESS_TOKEN = YOUR_API_KEY
GROQ_API_KEY= YOUR_API_KEY
```

```
cargo run 
```


### Todos: 

- [ ] Recursively Search for files
    - [ ] Need to Learn about Box Smart Pointers
- [ ] Implementing LLM via Api's( Groq or GoogleGenAi)
- [ ] Priortizing files 
    (ie: To reduce Api calls we have to priortize  
    which file content to serve LLLm to generate documentation)
- [ ] Seperating Different logics into Different Functions 
- [ ] Error Handling
    - [ ] Repo Not Found
    - [ ] Private Repo
    - [ ] Invalid Personal Access Token(ie: Not required in case of Public repos)
- [ ] Understanding and Implementing RAG from Scratch

