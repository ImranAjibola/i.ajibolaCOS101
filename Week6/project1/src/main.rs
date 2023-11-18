use std::io;

fn main() {
    for x in 0..150{
    println!("\nWhat is your name? ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed");
    name = name.trim().parse().expect("Failed");

    println!("What is your email address? ");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed");
    email = email.trim().parse().expect("Failed");

    println!("What is your Department? ");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed");
    department = department.trim().parse().expect("Failed");

    println!("What is your State of Origin? ");
    let mut origin = String::new();
    io::stdin().read_line(&mut origin).expect("Failed");
    origin = origin.trim().parse().expect("Failed");

    println!("What is your level are you in? ");
    let mut levels = String::new();
    io::stdin().read_line(&mut levels).expect("Failed");
    let level:i32 = levels.trim().parse().expect("Failed");

    println!("What is your CGPA? ");
    let mut gpa = String::new();
    io::stdin().read_line(&mut gpa).expect("Failed");
    let cgpa:f32 = gpa.trim().parse().expect("Failed");

    println!("Are you a class representative? ");
    let mut rep = String::new();
    io::stdin().read_line(&mut rep).expect("Failed");
    rep = rep.trim().parse().expect("Failed");

    if rep == "no" && level > 100 && cgpa > 4.0{
        println!("\nYou are eligible to vote
        \nStudent Information:
        \nName: {}
        \nEmail: {}
        \nDepartment: {}
        \nState of Origin: {}", name, email, department, origin);
    } else{
        println!("You are not eligible to vote");
    }}
}
