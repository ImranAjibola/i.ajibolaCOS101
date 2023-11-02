fn main() {
    let n1 = "Electrical".to_string();
    let n2 = "Electronics".to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3;
    println!("This is {}", n4);

    let w1 = "Computer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2;

    println!();
    println!("{}",w3);
}
