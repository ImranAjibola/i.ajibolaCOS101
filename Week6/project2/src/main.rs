use std::io;

fn main() {
    for x in 0..500{
        println!("\nHow many papers have you written?");
        let mut paper = String::new();
        io::stdin().read_line(&mut paper).expect("Failed");
        let papers:i32 = paper.trim().parse().expect("Failed");

        if papers >=3 && papers < 6{
            println!("\nYour incentive is N500,000");
        }
        else if papers >=6 && papers < 10{
            println!("\nYour incentive is N800,000");
        }
        else if papers >10{
            println!("\nYour incentive is N1,000,000");
        }
        else if papers <3 && papers >0{
            println!("\nYour incentive is N100,000");
        }
}}
