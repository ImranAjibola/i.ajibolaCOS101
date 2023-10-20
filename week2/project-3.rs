fn main(){
    let p:f64 = 210000.00;
    let r:f64 = 5.00;
    let n:f64 = 3.00;
    let b = p * (1.00 - (r / 100.00)).powf(n);
    
    println!("The value of the TV after 3 years {:.2}", b);
}