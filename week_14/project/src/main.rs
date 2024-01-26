use std::io::Read;
use std::io;
fn main(){
    println!("what is your user in the compnay");
    println!("if administrator(press a)
              if project manager(press p)
              if employee(press e)
              if customer(press c)
              if vendor(press v)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("unable to read_line");
    let user = input.trim();

    if user == "a" {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else
    if user == "p" {
    let mut file = std::fs::File::open("projects_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else
    if user == "e" {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else
    if user == "c" {                
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else
    if user =="v" {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else {
        println!("incorrect input");
    }
}
    
       
   