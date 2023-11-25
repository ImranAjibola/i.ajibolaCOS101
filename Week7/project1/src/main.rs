use std::io;

fn area_trapezium() {
    let mut height_trap = String::new();
    println!("Input the height of the trapezium");
    io::stdin().read_line(&mut height_trap).expect("Failed");
    let height_trapezium:f32 = height_trap.trim().parse().expect("Failed");

    let mut base_trap1 = String::new(); 
    println!("Input the first base of the trapezium");
    io::stdin().read_line(&mut base_trap1).expect("Failed");
    let base_trapezium1:f32 = base_trap1.trim().parse().expect("Failed");

    let mut base_trap2 = String::new();
    println!("Input the second base of the trapezium");
    io::stdin().read_line(&mut base_trap2).expect("Failed");
    let base_trapezium2:f32 = base_trap2.trim().parse().expect("Failed");

    let area_trap:f32 = height_trapezium / 2.0 * (base_trapezium1+base_trapezium2);
    println!("The area of the trapezium is : {}", area_trap);
}
fn area_rhombus() {
    let mut diagonal_rhom1 = String::new(); 
    println!("Input the first diagonal of the rhombus");
    io::stdin().read_line(&mut diagonal_rhom1).expect("Failed");
    let diagonal_rhombus1:f32 = diagonal_rhom1.trim().parse().expect("Failed");

    let mut diagonal_rhom2 = String::new(); 
    println!("Input the second diagonal of the rhombus");
    io::stdin().read_line(&mut diagonal_rhom2).expect("Failed");
    let diagonal_rhombus2:f32 = diagonal_rhom2.trim().parse().expect("Failed");

    let area_rhom:f32 = 0.5 * diagonal_rhombus1 * diagonal_rhombus2;
    println!("The area of the rhombus is : {}", area_rhom);
}
fn area_parallelogram() {
    let mut base_para = String::new(); 
    println!("Input the base of the parallelogram");
    io::stdin().read_line(&mut base_para).expect("Failed");
    let base_parallelogram:f32 = base_para.trim().parse().expect("Failed");

    let mut altitude_para = String::new(); 
    println!("Input the altitude of the parallelogram");
    io::stdin().read_line(&mut altitude_para).expect("Failed");
    let altitude_parallelogram:f32 = altitude_para.trim().parse().expect("Failed");

    let area_para:f32 = base_parallelogram * altitude_parallelogram;
    println!("The area of the parallelogram is : {}", area_para);
}
fn volume_cube() {
    let mut lenth_cub = String::new(); 
    println!("Input the lenght of the cube");
    io::stdin().read_line(&mut lenth_cub).expect("Failed");
    let lenth_cube:f32 = lenth_cub.trim().parse().expect("Failed");

    let vol_cube:f32 = 6.0 * lenth_cube.powf(2.0);
    println!("The volume of the cube is : {}", vol_cube);
}
fn volume_cylinder() {
    let mut radius_cyl = String::new(); 
    println!("Input the radius of the cylinder");
    io::stdin().read_line(&mut radius_cyl).expect("Failed");
    let radius_cylinder:f32 = radius_cyl.trim().parse().expect("Failed");

    let mut height_cyl = String::new(); 
    println!("Input the height of the cylinder");
    io::stdin().read_line(&mut height_cyl).expect("Failed");
    let height_cylinder:f32 = height_cyl.trim().parse().expect("Failed");

    let volume_cyl:f32 = (22.0/7.0) * radius_cylinder.powf(2.0) * height_cylinder;
    println!("The volume of the cylinder is : {}", volume_cyl);
}
fn main() {
    println!("Program to calculate volumes and areas of different shapes");
    println!("\nThis program calculates the following:
    a. Area of a Trapezium
    b. Area of a rhombus
    c. Area of a Parallelogram
    d. Volume of a cube
    e. volume of a cylinder
    ");
    println!("\nTo choose any of the above type in the following corresponding key for each of them 'a', 'b', 'c', 'd' and 'e'");

    let mut calculate = String::new();
    io::stdin().read_line(&mut calculate).expect("Failed");
    calculate = calculate.trim().parse().expect("Failed");

    if calculate == "a"{
        area_trapezium();
    }
    else if calculate == "b"{
        area_rhombus();
    }
    else if calculate == "c"{
        area_parallelogram();
    }
    else if calculate == "d"{
        volume_cube();
    }
    else if calculate == "e"{
        volume_cylinder();
    }
    else{
        println!("Input the correct key");
    }
}