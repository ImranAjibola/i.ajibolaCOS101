fn main() {
    let mut user = String::new();
    println!("\nEnter the number of users using this system");
    std::io::stdin().read_line(&mut user).expect("Failed");
    let no_users:i32 = user.trim().parse().expect("Failed");

    for x in 1..=no_users{
        println!("\nEnter your name");
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).expect("Failed");
        name = name.trim().parse().expect("Failed");

        println!("\nEnter your age");
        let mut age = String::new();
        std::io::stdin().read_line(&mut age).expect("Failed");
        let age:i32 = age.trim().parse().expect("Failed");

        println!("\nEnter email address");
        let mut email = String::new();
        std::io::stdin().read_line(&mut email).expect("Failed");
        email = email.trim().parse().expect("Failed");

        println!("Enter years of Experience");
        let mut experience = String::new();
        std::io::stdin().read_line(&mut experience).expect("Failed");
        let experience:i32 = experience.trim().parse().expect("Failed");
    }

}
