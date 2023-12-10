use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("Failed");
    println!("File is removed");
}