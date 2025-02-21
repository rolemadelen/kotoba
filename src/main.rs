// use std::io::{Write, stdin, stdout};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Word {
    kanji: String,
    kana: String,
    definition: String,
    sentence: String,
}

struct AppConfig {
    base_dir: String,
    file_name: String
}

impl AppConfig {
    fn new(path: &str, file: &str) -> AppConfig {
        AppConfig {
            base_dir: path.to_string(),
            file_name: file.to_string()
        }
    }
    
    fn init(&self) -> () {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let jpvoca_dir: PathBuf = home_dir.join(&self.base_dir);
        
        if !jpvoca_dir.exists() {
            create_dir_all(&jpvoca_dir).expect("Failed to create tracker directory");
            println!("jpvoca directory created at {:?}", jpvoca_dir);
        } else {
            println!("jpvoca directory exists");
        }
    }
    
    fn get_words(&self) -> Vec<Word> {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let file_path: PathBuf = home_dir.join(&self.base_dir).join(&self.file_name);

        if !file_path.exists() {
            println!("No existing data found.");
            return Vec::new();
        }

        let file = File::open(&file_path).expect("Failed to open data file");
        let reader = BufReader::new(file);

        let words_map: HashMap<String, Word> = serde_json::from_reader(reader).expect("Failed to parse JSON");

        words_map.into_values().collect()
    }
}

struct App {
    words: Vec<Word>
}

impl App {
    fn new(data: Vec<Word>) -> App {
        App {
            words: data
        }
    }
}

fn main() {
    let config = AppConfig::new(".jpvoca", "data.json");
    config.init();

    // let words = config.get_words();
    let app = App::new(config.get_words());
    for word in app.words {
        println!("{:?}", word);
    }
}
