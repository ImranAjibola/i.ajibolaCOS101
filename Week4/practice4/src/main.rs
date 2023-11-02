fn main() {
    let fullname = "Imran Olakunle Ajibola";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science".to_string();
    school.push_str(" and Technology");

    println!("My name is {}", fullname);
    println!("The length of my name is {}", fullname.len());
    println!("I am a student of {}", department);
    println!("{}", school);
    println!("{}", uni);
}
