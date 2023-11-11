use std::io;

fn main() {
    loop{
    println!("\nProgram to determine annual incentive");
    
    println!("\nIf experience type 'e' and if inexperienced type 'i'");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed");
    experience = experience.trim().parse().expect("Failed");
    
    println!("Enter your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed");
    let input2:i32 = age.trim().parse().expect("Failed");

    if experience == "e" && input2 >=40{
        println!("Your incentive is N1,560,000");
    }
    else if experience == "e" && input2 >=30 && input2 <40{
        println!("Your incentive is N1,480,000");
    }
    else if experience == "e" && input2 <30{
        println!("Your incentive is N1,300,000");
    }
    else if experience == "i"{
        println!("Your incentive is N100,000");
    }
    else{
        println!("Use the correct keys 'e' and 'i' for your experience input!");
    }
}
} 