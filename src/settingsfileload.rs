use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Settings {
    DefaultSaveFile: String,
    DefaultWindowWidth: i32,
    DefaultWindowHeight: i32,
    GUIScale: u8,
    FPSCap: bool
}

fn LoadSettings() -> Settings {
    // read JSON file into string
    let mut file = File::open("res/settings.json")
        .expect("Cannot open settings file");

    let mut raw_data = String::new();
    file.read_to_string(&mut raw_data);

    // parse JSON in string
    let res = serde_json::from_str(&raw_data);

    // default settings
    let mut ret: Settings = Settings{
        DefaultSaveFile: "res/test.gtsave".to_string(),
        DefaultWindowWidth: 800,
        DefaultWindowHeight: 600,
        GUIScale: 1,
        FPSCap: true
    };

    // copy JSON to struct
    if res.is_ok() {
        ret = res.unwrap();
    } else {
        println!("Cannot read data in settings file");
    }
    return ret;
}
