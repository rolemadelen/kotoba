use std::io::{Write, stdin, stdout};
use std::fs::{create_dir_all};
use std::path::PathBuf;

fn get_input() -> i32 {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<i32>().unwrap()
}

fn main() {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let jpvoca_dir: PathBuf = home_dir.join(".jpvoca");
    if !jpvoca_dir.exists() {
        create_dir_all(&jpvoca_dir).expect("Failed to create tracker directory");
        println!("jpvoca directory created at {:?}", jpvoca_dir);
    } else {
        println!("jpvoca directory exists");
    }

    println!("1. Add a word");
    println!("2. Delete a word");
    println!("3. Review");
    println!("0. Quit");
    print!("> ");
    let _ = stdout().flush();

    let option = get_input();

    println!("option: {}", option);
}
