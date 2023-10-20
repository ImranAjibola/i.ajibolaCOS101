fn main(){
    let p:f64 = 1000.0;
    let r:f64 = 1.0;
    let t:f64 = 2.0;

    // simple intrest
    let a = p*(1.0 + (r/100.0))*t;
    println!("the amount is {}", a);
    let si = a-p;
    println!("simple Intest is {}", si);
}