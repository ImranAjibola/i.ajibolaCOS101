use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid number");

    println!("Enter your Age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid number");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 18{
        println!("Welcome to the party {}!", input1);
    } else {
        println!("You are not eligible to enter the party {}", input1);
    }
}
