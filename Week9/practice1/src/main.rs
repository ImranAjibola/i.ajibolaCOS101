use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust file Input & output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("Failed");
    file.write_all("Welcome to rust programming\n".as_bytes()).expect("Failed");
    file.write_all(announce.as_bytes()).expect("Failed");
    file.write_all(dept.as_bytes()).expect("Failed");
    println!("\nData written to file");
}
