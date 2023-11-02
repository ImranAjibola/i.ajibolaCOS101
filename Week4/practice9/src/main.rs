fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of A: {}", A);
    println!("Value of B: {}", B);

    let mut res = A > B;
    println!("A greater than B: {}", res);

    res = A < B;
    println!("A less than B: {} ", res);
    
    res = A >= B;
    println!("A is greater than or equals to B: {} ", res);

    res = A <= B;
    println!("A is less than or equals to B: {} ", res);

    res = A == B;
    println!("A is equals to B: {} ", res);

    res = A != B;
    println!("A is not equals to B: {} ", res);

}
