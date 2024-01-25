use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    println!("Are you an administrator, project manager, employee, customer or vendor?");
    io::stdin().read_line(&mut input).expect("Failed");
    input = input.trim().parse().expect("Failed");

    if input == "administrator"{
        admin();
    }
    else if input == "project manager"{
        project_manager();
    }
    else if input == "employee"{
        employee();
    }
    else if input == "customer"{
        customer();
    }
    else if input == "vendor"{
        vendor();
    }
    else{
        println!("\nPlease type any of the following: administrator, project manager, employee, customer or vendor");
    }

}

fn admin() {
    let mut file_admin = std::fs::File::open(r"C:\Users\Imraan\OneDrive\Documents\globacom_dbase.sql").unwrap();
    let mut contents_admin = String::new();
    file_admin.read_to_string(&mut contents_admin).unwrap();
    print!("{}", contents_admin);
}

fn project_manager() {
    let mut file_project = std::fs::File::open(r"C:\Users\Imraan\OneDrive\Documents\globacom_project.sql").unwrap();
    let mut contents_project = String::new();
    file_project.read_to_string(&mut contents_project).unwrap();
    print!("{}", contents_project);
}

fn employee() {
    let mut file_employee = std::fs::File::open(r"C:\Users\Imraan\OneDrive\Documents\globacom_staff.sql").unwrap();
    let mut contents_employee = String::new();
    file_employee.read_to_string(&mut contents_employee).unwrap();
    print!("{}", contents_employee);
}

fn customer() {
    let mut file_customer = std::fs::File::open(r"C:\Users\Imraan\OneDrive\Documents\globacom_customer.sql").unwrap();
    let mut contents_customer = String::new();
    file_customer.read_to_string(&mut contents_customer).unwrap();
    print!("{}", contents_customer);
}

fn vendor() {
    let mut file_vendor = std::fs::File::open(r"C:\Users\Imraan\OneDrive\Documents\globacom_vendor.sql").unwrap();
    let mut contents_vendor = String::new();
    file_vendor.read_to_string(&mut contents_vendor).unwrap();
    print!("{}", contents_vendor);
}