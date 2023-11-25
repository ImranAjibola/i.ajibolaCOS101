use std::io;

fn checker() {
    let mut input = String::new();
    println!("Enter a character");
    io::stdin().read_line(&mut input).expect("Failed");
    let ch:char = input.trim().parse().expect("Failed");

    if ch >= '0' && ch <= '9'{
        println!("Character '{}' is a digit", ch);
    }
    else{
        println!("Character '{}' is not a digit", ch);
    }
}

fn main() {
    println!("Welcome, this program checks whether a character is a digit or not");
    checker();
}
