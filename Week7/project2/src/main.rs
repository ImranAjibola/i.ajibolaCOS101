use std::io;

fn main() {
    println!("Program to dispay family information");

    println!("\nHow many siblings do you have?");
    let mut siblings = String::new();
    io::stdin().read_line(&mut siblings).expect("Failed");
    let no_siblings:i32 = siblings.trim().parse().expect("Failed");

    for sibling in 1..no_siblings+1{
        println!("Enter information for sibling {}:", sibling);

        println!("\nWhat is the name of sibling {}:", sibling);
        let mut name_siblings = String::new();
        io::stdin().read_line(&mut name_siblings).expect("Failed");
        name_siblings = name_siblings.trim().parse().expect("Failed");

        println!("\nWhat is the age of sibling {}:", sibling);
        let mut age_siblings = String::new();
        io::stdin().read_line(&mut age_siblings).expect("Failed");
        let age_siblings:i32 = age_siblings.trim().parse().expect("Failed");

        if age_siblings >= 18{
            println!("Are you single or married?");
            let mut is_single = String::new();
            io::stdin().read_line(&mut is_single).expect("Failed");
            is_single = is_single.trim().parse().expect("Failed");
    
            if is_single == "single"{
                println!("Are you a student or a worker?");
                let mut is_student = String::new();
                io::stdin().read_line(&mut is_student).expect("Failed");
                is_student = is_student.trim().parse().expect("Failed");
    
                if is_student == "student"{
                    println!("What university do you attend?");
                    let mut uni = String::new();
                    io::stdin().read_line(&mut uni).expect("Failed");
                    uni = uni.trim().parse().expect("Failed");
    
                    println!("What course are you studying?");
                    let mut course = String::new();
                    io::stdin().read_line(&mut course).expect("Failed");
                    course = course.trim().parse().expect("Failed");
                }
            }
            if is_single == "married"{
                println!("Do they children?");
                let mut children = String::new();
                io::stdin().read_line(&mut children).expect("Failed");
                children = children.trim().parse().expect("Failed");
    
                println!("What city does their family live in?");
                let mut city = String::new();
                io::stdin().read_line(&mut city).expect("Failed");
                city = city.trim().parse().expect("Failed");
            }
        }
        if age_siblings < 18{
            println!("Have they written WAEC?");
            let mut waec = String::new();
            io::stdin().read_line(&mut waec).expect("Failed");
            waec = waec.trim().parse().expect("Failed");
    
            if waec == "yes"{
                println!("What secondary school did they attend?");
                let mut secondary = String::new();
                io::stdin().read_line(&mut secondary).expect("Failed");
                secondary = secondary.trim().parse().expect("Failed");
            }
            if waec == "no"{
                println!("What class are they currently in?");
                let mut class = String::new();
                io::stdin().read_line(&mut class).expect("Failed");
                class = class.trim().parse().expect("Failed");
            }
        }
    }
    println!("\nAll the details of the client's siblings");

}