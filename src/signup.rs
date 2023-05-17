use reqwest::blocking::Client;

const BASE_SIGNUP_GENIUS_URL: &str = "https://api.signupgenius.com/v2/k";

#[derive(Debug)]
pub struct SignUp {
    pub url: String,
    pub id: u64,
    pub title: String,
    pub author: String,
    pub roles: Vec<SignUpRole>,
}

impl SignUp {
    pub fn new(url: &str, id: u64, title: &str, author: &str) -> Self {
        SignUp {
            url: url.to_string(),
            id,
            title: title.to_string(),
            author: author.to_string(),
            roles: Vec::new(),
        }
    }

    pub fn fill_roles(
        &mut self,
        blocking_client: &Client,
        signup_genius_token: &str,
    ) -> &Vec<SignUpRole> {
        if let Ok(new_roles) =
            get_signup_roles_available(blocking_client, signup_genius_token, self.id)
        {
            self.roles = new_roles;
        }

        &self.roles
    }
}

#[derive(Debug)]
pub struct SignUpRole {
    pub title: String,
    pub needed_count: u64,
    pub signup_id: u64,
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

pub fn get_active_signups_with_roles(
    blocking_client: &Client,
    signup_genius_token: &str,
) -> Result<Vec<SignUp>, reqwest::Error> {
    let mut current_signups = get_active_signups(blocking_client, signup_genius_token)?;

    for s in current_signups.iter_mut() {
        s.fill_roles(blocking_client, signup_genius_token);
    }

    Ok(current_signups)
}

pub fn get_active_signups(
    blocking_client: &Client,
    signup_genius_token: &str,
) -> Result<Vec<SignUp>, reqwest::Error> {
    let mut signups: Vec<SignUp> = Vec::new();

    if let Some(active_json_array) = get_json_response(
        blocking_client,
        &format!("{}/signups/created/active/", BASE_SIGNUP_GENIUS_URL),
        Some(vec![("user_key", signup_genius_token)]),
    )?["data"]
        .as_array()
    {
        for i in 0..active_json_array.len() {
            let signup_data = &active_json_array[i];
            signups.push(SignUp::new(
                signup_data["signupurl"].as_str().unwrap(),
                signup_data["signupid"].as_u64().unwrap(),
                signup_data["title"].as_str().unwrap(),
                signup_data["contactname"].as_str().unwrap(),
            ));
        }
    }

    Ok(signups)
}

pub fn get_signup_roles_available(
    blocking_client: &Client,
    signup_genius_token: &str,
    signup_id: u64,
) -> Result<Vec<SignUpRole>, reqwest::Error> {
    let mut roles: Vec<SignUpRole> = Vec::new();

    if let Some(roles_array_json) = get_json_response(
        blocking_client,
        &format!(
            "{}/signups/report/available/{}/",
            BASE_SIGNUP_GENIUS_URL, signup_id
        ),
        Some(vec![("user_key", signup_genius_token)]),
    )?["data"]["signup"]
        .as_array()
    {
        for i in 0..roles_array_json.len() {
            let role_data = &roles_array_json[i];
            roles.push(SignUpRole {
                title: role_data["item"].as_str().unwrap().to_string(),
                needed_count: role_data["myqty"].as_u64().unwrap(),
                signup_id,
            });
        }
    }

    Ok(roles)
}
