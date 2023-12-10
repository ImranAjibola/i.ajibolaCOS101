use std::io::Write;
fn main() {
    let name = vec!["Student Name", "Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric = vec!["Matric. Number", "ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let dept = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec![300,100,200,200,100];
    
    let mut file = std::fs::File::create("project2.txt").expect("Failed");

    for i in 0..name.len(){
        println!("Name:{}\nMatric Number:{}\nDepartment:{}\nlevel:{}", name[i], matric[i], dept[i], level[i]);

        file.write_all(format!("{}\t", name[i]).as_bytes()).expect("Failed");
        file.write_all(format!("{}\t", matric[i]).as_bytes()).expect("Failed");
        file.write_all(format!("{}\t", dept[i]).as_bytes()).expect("Failed");
        file.write_all(format!("{}\n", level[i  ]).as_bytes()).expect("Failed");   

        println!("done");



    }
    
}
