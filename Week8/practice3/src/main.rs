fn value(n:Option<&char>){
    println!("Element of vector {:?}", n);
}

fn main() {
    let v = vec!['R', 'U', 'S', 'T', 'A', 'I', 'A', 'N'];
    
    let mut input1 = String::new();
    println!("\nEnter an index value btw (0-8)");
    std::io::stdin().read_line(&mut input1).expect("Failed");
    
    let index:usize = input1.trim().parse().expect("Failed");
    let ch: Option<&char> = v.get(index);
    value(ch);
}
