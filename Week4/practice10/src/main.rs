fn main() {
    let a = 20;
    let b = 10;

    if (a > 10) && (b > 20) {
        println!("True");
    }

    let c = 0;
    let d = 30;

    if (c>10) || (d<10) {
        println!("False");
    }

    let is_elder = false;

    if !is_elder {
        println!("Not elder")
    }

}
