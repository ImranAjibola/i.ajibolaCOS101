fn main(){
    let p:f64 = 520000000.00;
    let r:f64 = 10.00;
    let n:f64 = 5.00;
    let b = p * (1.00 + (r / 100.00)).powf(n);
    let ci = b - p;
    
    println!("The compound interest for 5 years at 10% per annum is {:.2}", ci);
}