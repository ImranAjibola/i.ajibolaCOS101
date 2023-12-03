fn main() {
    println!("Public Service APS level checker for the Federal Government of Nigeria.");

    let mut user = String::new();
    println!("\nEnter the number of users using this system");
    std::io::stdin().read_line(&mut user).expect("Failed");
    let no_users:i32 = user.trim().parse().expect("Failed");

    for x in 1..=no_users{
        let main = vec!["Office Administrator", "Academic", "Lawyer", "Teacher"];
        let mut option = String::new();
        println!("\n choose the following 0- Office Administrator, 1- Academic, 2- Lawyer, 3- Teacher");
        std::io::stdin().read_line(&mut option).expect("Failed");
        let index:i32 = option.trim().parse().expect("Failed");

        if index == 0{
            println!("\n choose the following 0- Intern, 1-Administrator, 2-senior Adminstrator, 3- Office Manager, 4- Director, 5- CEO");
            let a = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
            let mut a1 = String::new();
            std::io::stdin().read_line(&mut a1).expect("Failed");
            let a1:i32 = a1.trim().parse().expect("Failed");
            if a1 == 0{
                println!("You are a public servant APS 1-2");
            }
            if a1 == 1{
                println!("You are a public servant APS 3-5");
            }
            if a1 == 2{
                println!("You are a public servant APS 5-8");
            }
            if a1 == 3{
                println!("You are a public servant EL1 8-10");
            }
            if a1 == 4{
                println!("You are a public servant EL2 10-13");
            }
            if a1 == 5{
                println!("You are a public servant SES");
            }
        }
        if index == 1{
            println!("\n choose the following 1-Research Assistant, 2-PhD Candidate, 3- Post-Doc Researcher, 4- Senior Lecturer, 5- Dean");
            let b = vec!["--", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
            let mut b1 = String::new();
            std::io::stdin().read_line(&mut b1).expect("Failed");
            let b1:i32 = b1.trim().parse().expect("Failed");
            if b1 == 0{
                println!("You are a public servant APS 1-2");
            }
            if b1 == 1{
                println!("You are a public servant APS 3-5");
            }
            if b1 == 2{
                println!("You are a public servant APS 5-8");
            }
            if b1 == 3{
                println!("You are a public servant EL1 8-10");
            }
            if b1 == 4{
                println!("You are a public servant EL2 10-13");
            }
            if b1 == 5{
                println!("You are a public servant SES");
            }
        }
        if index == 2{
            println!("\n choose the following 0- Paralegal, 1-Junior Associate, 2-Associate, 3- Senior Associate 1-2, 4- Senior Associate 3-4, 5- Partner");
            let c = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
            let mut c1 = String::new();
            std::io::stdin().read_line(&mut c1).expect("Failed");
            let c1:i32 = c1.trim().parse().expect("Failed");
            if c1 == 0{
                println!("You are a public servant APS 1-2");
            }
            if c1 == 1{
                println!("You are a public servant APS 3-5");
            }
            if c1 == 2{
                println!("You are a public servant APS 5-8");
            }
            if c1 == 3{
                println!("You are a public servant EL1 8-10");
            }
            if c1 == 4{
                println!("You are a public servant EL2 10-13");
            }
            if c1 == 5{
                println!("You are a public servant SES");
            }
        }
        if index == 3{
            println!("\n choose the following 0- Placement, 1-Classroom Teacher, 2-Snr Teacher, 3- Leading Teacher, 4- Deputy Principal, 5- Principal");
            let d = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
            let mut d1 = String::new();
            std::io::stdin().read_line(&mut d1).expect("Failed");
            let d1:i32 = d1.trim().parse().expect("Failed");
            if d1 == 0{
                println!("You are a public servant APS 1-2");
            }
            if d1 == 1{
                println!("You are a public servant APS 3-5");
            }
            if d1 == 2{
                println!("You are a public servant APS 5-8");
            }
            if d1 == 3{
                println!("You are a public servant EL1 8-10");
            }
            if d1 == 4{
                println!("You are a public servant EL2 10-13");
            }
            if d1 == 5{
                println!("You are a public servant SES");
            }
        }
    }
}
