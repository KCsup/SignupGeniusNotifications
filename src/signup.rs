use reqwest::blocking::Client;

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
