use std::io;

fn main() {
    println!("Enter a number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid number");
    let mut num:i32 = input.trim().parse().expect("Not valid");
    
    while num >10{
        println!("Inside loop number value {}", num);
        num+=1;
    }
    println!("Outside loop number is {}", num);
}
