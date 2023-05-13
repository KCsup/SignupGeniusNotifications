use reqwest::{
    blocking::{Client, Response},
    Error,
};

pub fn send_announcement(
    blocking_client: &Client,
    base_url: &str,
    access_token: &str,
    course_id: u32,
    title: &str,
    message: &str,
    is_published: bool,
) -> Result<Response, Error> {
    blocking_client
        .post(format!("{base_url}/courses/{course_id}/discussion_topics"))
        .query(&[
            ("access_token", access_token.to_string()),
            ("title", title.to_string()),
            ("message", message.to_string()),
            ("published", is_published.to_string()),
            ("is_announcement", true.to_string()),
        ])
        .send()
}
