use std::io;

struct Laptop {
    name: String,
    quantity: i32,
    amount: i32,
    price: i32,
}

impl Laptop {
    fn total(&self) -> i32 {
        self.amount * self.price
    }
}

impl Laptop {
    fn remaining(&self) -> i32 {
        self.quantity - self.amount
    }
}

fn display(pc: Laptop) {
    println!("Name: {} \nRequested Amount: {} \nEstimated Price: {} \nProucts Remaining: {}\n",pc.name,pc.amount,pc.total(),pc.remaining());
}

fn main() {
    let mut input1 = String::new();
    println!("How many products do you want?");
    io::stdin().read_line(&mut input1).expect("Failed");
    let num:i32 = input1.trim().parse().expect("Failed");
    
    let hp = Laptop {
        name:String::from("Hewlett Packard"),
        quantity:10,
        amount:num,
        price:650000,
    };
    let ibm = Laptop {
        name:String::from("IBM"),
        quantity:6,
        amount:num,
        price:755000,
    };
    let toshiba = Laptop {
        name:String::from("Toshiba"),
        quantity:10,
        amount:num,
        price:550000,
    };
    let dell = Laptop {
        name:String::from("Dell"),
        quantity:4,
        amount:num,
        price:850000,
    };

    let sum = hp.total() + ibm.total() + toshiba.total() + dell.total();

    display(hp);
    display(ibm);
    display(toshiba);
    display(dell);

    println!("Your total is: N{}",sum);

}