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
    let settings_keys = ["canvas_token", "base_canvas_url", "signup_genius_token"];

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

    if let Some(sg_key) = settings_map.get("signup_genius_token") {
        let current_signups =
            signup::get_active_signups_with_roles(&blocking_client, &sg_key).unwrap();
        println!("Current Signups: {:?}", current_signups);
    }
}
