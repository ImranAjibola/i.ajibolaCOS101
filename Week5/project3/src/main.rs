use std::io;

fn main() {
    println!("The menu is below to order answer the below questions to get your total price");
    println!("\n
        Menu			              Price
 Poundo Yam/Edinkaiko Soup 		     N3,200
 Fried Rice & Chicken 			     N3,000
 Amala & Ewedu Soup			     N2,500
 Eba & Egusi Soup			     N2,000
 White Rice & Stew			     N2,500
 ");
    
    println!("What quantity of Poundo Yam/Edinkaiko Soup do you want?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed");
    let input1:f32 = input1.trim().parse().expect("Failed");

    println!("What quantity of Fried Rice & Chicken do you want?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed");
    let input2:f32 = input2.trim().parse().expect("Failed");

    println!("What quantity of Amala & Ewedu Soup do you want?");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed");
    let input3:f32 = input3.trim().parse().expect("Failed");

    println!("What quantity of Eba & Egusi Soup do you want?");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Failed");
    let input4:f32 = input4.trim().parse().expect("Failed");

    println!("What quantity of White Rice & Stew do you want?");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Failed");
    let input5:f32 = input5.trim().parse().expect("Failed");

    let p:f32 = 3200.0*input1;
    let f:f32 = 3000.0*input2;
    let a:f32 = 2500.0*input3;
    let e:f32 = 2000.0*input4;
    let w:f32 = 2500.0*input5;
    let total = p+f+a+e+w;

    if total >= 10000.0{
        let r#final:f32 = total * 0.05;
        let finale = total - r#final;
        println!("Your total is N{}", finale);
    }
    else{
        println!("Your total is N{}", total);
    }
}