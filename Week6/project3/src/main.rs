use std::io;

fn main() {
    for x in 0..15{
    println!("multiplication table");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed");
    let input1:i32 = input1.trim().parse().expect("Failed");

    let mul1:i32 = input1 * 1;
    let mul2:i32 = input1 * 2;
    let mul3:i32 = input1 * 3;
    let mul4:i32 = input1 * 4;
    let mul5:i32 = input1 * 5;
    let mul6:i32 = input1 * 6;
    let mul7:i32 = input1 * 7;
    let mul8:i32 = input1 * 8;
    let mul9:i32 = input1 * 9;
    let mul10:i32 = input1 * 10;
    let mul11:i32 = input1 * 11;
    let mul12:i32 = input1 * 12;

    println!("\n{} * 1 = {}", input1, mul1);
    println!("{} * 2 = {}", input1, mul2);
    println!("{} * 3 = {}", input1, mul3);
    println!("{} * 4 = {}", input1, mul4);
    println!("{} * 5 = {}", input1, mul5);
    println!("{} * 6 = {}", input1, mul6);
    println!("{} * 7 = {}", input1, mul7);
    println!("{} * 8 = {}", input1, mul8);
    println!("{} * 9 = {}", input1, mul9);
    println!("{} * 10 = {}", input1, mul10);
    println!("{} * 11 = {}", input1, mul11);
    println!("{} * 12 = {}", input1, mul12);
}}
