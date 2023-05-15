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
    query_map: Option<Vec<(&str, &str)>>,
) -> Result<serde_json::Value, reqwest::Error> {
    let mut request_builder = blocking_client.get(url);

    if let Some(queries) = query_map {
        request_builder = request_builder.query(&queries);
    }

    request_builder.send()?.json::<serde_json::Value>()
}

// Theoretical method; Needs testing with the API to validate functionality
pub fn get_active_signup_ids(
    blocking_client: &Client,
    signup_genius_token: &str,
) -> Result<Vec<u64>, reqwest::Error> {
    let mut ids: Vec<u64> = Vec::new();

    if let Some(active_json_array) = get_json_response(
        blocking_client,
        &format!("{}/signups/created/active/", BASE_SIGNUP_GENIUS_URL),
        Some(vec![("user_key", signup_genius_token)]),
    )?
    .as_array()
    {
        for i in 0..active_json_array.len() {
            ids.push(active_json_array[i]["signupid"].as_u64().unwrap());
        }
    }

    Ok(ids)
}
