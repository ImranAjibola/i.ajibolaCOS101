use std::io;

fn main() {
    println!("This is a Program to calculate roots of a quadratic equation and it's nature");

    println!("\nInput a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed");
    let a:f64 = input1.trim().parse().expect("Failed");

    println!("Input b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed");
    let b:f64 = input2.trim().parse().expect("Failed");
    
    println!("Input c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed");
    let c:f64 = input3.trim().parse().expect("Failed");

    let d:f64 = (b.powf(2.0)) - (4.0*a*c);
    if d > 0.0{
        println!("It has two distinct roots");
    }
    else if d == 0.0{
        println!("It has one real root");
    }
    else if d < 0.0{
        println!("It has no real roots");
    }

    let x1:f64 = (-(b) + d.sqrt())/ 2.0 * a;
    let x2:f64 = (-(b) - d.sqrt())/ 2.0 * a;
    
    println!("The root of the equation is {} and {}", x1, x2);
}