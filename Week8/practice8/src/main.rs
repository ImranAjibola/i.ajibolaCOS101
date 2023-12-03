fn main() {
    let mut mountain_height = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}", mountain_height);

    mountain_height.2 = "Lhotse";
    mountain_height.3 = 8516;

    println!("Changed tuple = {:?}", mountain_height);
}
