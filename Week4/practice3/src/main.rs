fn main() {
    let name1 = "Imran Ajibola";
    println!("My name is {}", name1);

    let name2 = name1.replace("Ajibola", "Olakunle");
    println!("You can also call me {}", name2);

    let faculty = "Faculty of Science and Technology";
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of {}", school);
}
