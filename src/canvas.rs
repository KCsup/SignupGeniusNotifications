use reqwest::{
    blocking::{Client, Response},
    Error,
};

pub fn send_announcement(
    access_token: &str,
    course_id: u32,
    title: &str,
    message: &str,
    is_published: bool,
) -> Result<Response, Error> {
    const BASE_CANVAS_URL: &str = "https://dexterschools.instructure.com/api/v1";
    let url = format!("{}/courses/{course_id}/discussion_topics", BASE_CANVAS_URL);

    let client = Client::new();

    let resp = client
        .post(url)
        .query(&[
            ("access_token", access_token.to_string()),
            ("title", title.to_string()),
            ("message", message.to_string()),
            ("published", is_published.to_string()),
            ("is_announcement", true.to_string()),
        ])
        .send()?;

    Ok(resp)
}
