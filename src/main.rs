use config::Config;

mod canvas;
mod signup;

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build();

    let mut canvas_token: Option<String> = None;

    if let Ok(conf) = settings {
        println!("File Found");
        let canvas_token_fetch = conf.get_string("canvas_token");
        if let Ok(t) = canvas_token_fetch {
            println!("{}", t);
            println!("Token Found");
            canvas_token = Some(t);
        }
    } else {
        println!("Error getting file");
    }

    if let Some(token) = canvas_token {
        let resp = canvas::send_announcement(
            token.as_str(),
            6768,
            "Another Rust Test",
            "This message was sent with the Rust Programming Language, and using proper token config",
            true,
        );

        if let Ok(resp_content) = resp {
            println!("response was ok!");
            println!("{:?}", resp_content);
        }
    }
}
