use reqwest::blocking::{Client, Response};

pub struct SignUp {
    pub url: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub roles: Vec<SignUpRole>,
}

pub struct SignUpRole {
    pub title: String,
    pub current_count: u32,
    pub needed_count: u32,
    pub location: String,
    // TODO: Date and Time stuff
}

// TODO: method for getting a SignUp with the SignupGenius API
pub fn get_json_response(
    blocking_client: &Client,
    url: &str,
) -> reqwest::Result<serde_json::Value> {
    let request = blocking_client.get("http://httpbin.org/ip").send();
    if let Ok(response) = request {} // Will not compile, needs to return right type
}
