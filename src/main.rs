use std::io::{Write, stdin, stdout};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::exit;

fn get_input() -> i32 {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<i32>().unwrap()
}

fn show_menu() -> i32 {
    println!("1. Add a word");
    println!("2. Delete a word");
    println!("3. Review");
    println!("0. Quit");
    print!("> ");
    let _ = stdout().flush();

    get_input()
}

fn add_word() -> Option<i32> {
    println!("add word");
    Some(1)
}

fn delete_word() -> Option<i32> {
    println!("delete word");
    Some(2)
}

fn review() -> Option<i32> {
    println!("review");
    Some(3)
}

fn init() -> () {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let jpvoca_dir: PathBuf = home_dir.join(".jpvoca");
    if !jpvoca_dir.exists() {
        create_dir_all(&jpvoca_dir).expect("Failed to create tracker directory");
        println!("jpvoca directory created at {:?}", jpvoca_dir);
    } else {
        println!("jpvoca directory exists");
    }
}

fn main() {
    init();

    let Some(option) = (match show_menu() {
        1 => add_word(),
        2 => delete_word(),
        3 => review(),
        _ => Some(-1),
    }) else {
        println!("NONE");
        todo!()
    };

    if option == -1 {
        println!("thank you");
        exit(1);
    }
}
