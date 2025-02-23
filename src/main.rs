use std::io::{Write, stdin, stdout};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

fn read_int() -> i32 {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    buf.trim().parse::<i32>().unwrap()
}

fn read_str() -> String {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    buf.trim().to_owned()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Word {
    kanji: String,
    kana: String,
    definition: String,
    sentence: String,
}

impl Word {
    fn new() -> Word {
        print!("Word (kanji): ");
        let _ = stdout().flush();
        let kanji = read_str();
        
        print!("Word (kana): ");
        let _ = stdout().flush();
        let kana = read_str();
        
        print!("Definition: ");
        let _ = stdout().flush();
        let definition = read_str();
        
        print!("Example sentence: ");
        let _ = stdout().flush();
        let sentence = read_str();

        Word { kanji, kana, definition, sentence }
    }

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
        }
    }
    
    fn get_words(&self) -> Vec<Word> {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let file_path= home_dir.join(&self.base_dir).join(&self.file_name);

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
    words: Vec<Word>,
    is_running: bool
}

impl App {
    fn new(data: Vec<Word>) -> App {
        App {
            words: data,
            is_running: true
        }
    }

    fn display_menu(&self) -> i32 {
        println!("1. Add a word");
        println!("2. Delete a word");
        println!("3. Review");
        println!("99. Quit");
        print!("> ");
        let _ = stdout().flush();

        read_int()
    }

    fn find(&self, target: &str) -> (usize, bool) {
        for (idx, word) in self.words.iter().enumerate() {
            if word.kanji == target || word.kana == target {
                return (idx, true);
            }
        }

        (0, false)
    }

    fn add_word(&mut self) { 
        let word = Word::new();
        self.words.push(word);
    }
    
    fn delete_word(&mut self) { 
        print!("Enter a word to delete: ");
        let _ = stdout().flush();
        let target = read_str();

        let (idx, is_found) = self.find(&target);
        if !is_found {
            println!("'{}' not in the list.", target);
        } else {
            println!("found at {}", idx);
            self.words.remove(idx);
        }
        
    }

    fn review(&self) { 
        println!("review");
    }

    fn save_words(&mut self) {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let file_path: PathBuf = home_dir.join(".jpvoca").join("data.json");

        // Read existing words
        let words_map: HashMap<String, Word> = self.words
            .iter()
            .enumerate()
            .map(|(i, w)| (i.to_string(), w.clone()))
            .collect();

        let file = File::create(&file_path).expect("Failed to open data file for writing");
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &words_map).expect("Failed to write to JSON");
    }

}

fn main() {
    let config = AppConfig::new(".jpvoca", "data.json");
    config.init();

    let mut app = App::new(config.get_words());
    
    for word in &app.words {
        println!("{:?}", word);
    }

    while app.is_running == true {
        let op = app.display_menu();
        println!("running {}", op);

        match op {
            1 => app.add_word(),
            2 => app.delete_word(),
            3 => app.review(),
            _ => app.is_running = false
        }
    }

    app.save_words();
    println!("terminated");
}
