use config::Config;
use reqwest;
use std::collections::HashMap;

mod canvas;
mod signup;

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build();

    let blocking_client = reqwest::blocking::Client::new();

    let mut settings_map: HashMap<String, String> = HashMap::new();
    let settings_keys = ["canvas_token", "base_canvas_url"];

    if let Ok(conf) = settings {
        println!("Config File Found...");
        for key in settings_keys {
            if let Ok(s) = conf.get_string(key) {
                settings_map.insert(key.to_string(), s.to_string());
            }
        }
    } else {
        println!("Error getting config file");
    }

    println!("{:?}", settings_map);

    // if let (Some(token), Some(base_url)) = (
    //     settings_map.get(settings_keys[0]),
    //     settings_map.get(settings_keys[1]),
    // ) {
    //     let resp = canvas::send_announcement(
    //         &blocking_client,
    //         &base_url,
    //         &token,
    //         6768,
    //         "Another Rust Test, with generalized config",
    //         "This message was sent with the Rust Programming Language, and using proper token config",
    //         true,
    //     );

    //     if let Ok(resp_content) = resp {
    //         println!("Status: {:?}", resp_content.status());
    //     }
    // }

    let json = signup::get_json_response(&blocking_client, "https://httpbin.org/json");
    println!("json: {:?}", json);
}
