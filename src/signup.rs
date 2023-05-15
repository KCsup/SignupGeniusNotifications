use reqwest::blocking::Client;

const BASE_SIGNUP_GENIUS_URL: &str = "https://api.signupgenius.com/v2/k";

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
) -> Result<serde_json::Value, reqwest::Error> {
    blocking_client.get(url).send()?.json::<serde_json::Value>()
}

// Theoretical method; Needs testing with the API to validate functionality
fn get_active_signups(
    blocking_client: &Client,
    signup_genius_user_key: &str,
) -> Result<Vec<SignUp>, reqwest::Error> {
    let active_json = get_json_response(
        blocking_client,
        &format!("{}/signups/created,active", BASE_SIGNUP_GENIUS_URL),
    ); // Will not compile; type return doesn't match
}
