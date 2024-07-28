use serde::Deserialize;
use reqwest::Error;
use reqwest::header::HeaderMap;
use reqwest::header::USER_AGENT;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::HeaderValue;


// This is going to pull values from the json
// Attached to the marcro 
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
    avatar_url: String,
}

// This contructs the headers
// Github returns a 400 if you don't set the User Agent
fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
    headers
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                        owner= "rust-lang-nursery",
                        repo = "rust-cookbook");
    println!("{}", request_url);
// Need to use the client contructor to have multiple headers. 
    let client = reqwest::Client::new();
    let response = client
    .get(&request_url)
    .headers(construct_headers())
    .send()
    .await?;
    // Creates a Vector based on the structure of the User object. 
    // It gets the json of the response variable and then sends that to the User object which triggers a macro and is returned as a vec 
    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    println!("There are {}, stargazers for the repo", users.len());
    Ok(())
}