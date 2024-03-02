use std::io;

struct Product {
    name : String,
    description: String,
    price: f64,
    quantity: i32,
}

fn main() {

    // let mut products : Vec<i32> = Vec::new();

    if !admin_check() {
        println!("Sorry, wrong credentials :<\nExiting System!");
        return;
    }

    // println!("\tRusty Inventory Management\n----------------------------------------
    // => Add Product
    // => Edit Product Info
    // => Delete Product
    // ");



}

fn admin_check() -> bool {

    // For the sake of simplicity, I have set admin as login user and password!
    let user_check_val : String = String::from("admin");
    let pass_check_val : String = String::from("admin");

    println!("\n----------------------------------------\n\tRusty Inventory Management\n----------------------------------------");
    println!("\t      'Login Panel'\n");

    let mut user_name = String::new();
    println!("Enter your username: ");
    io::stdin().read_line(&mut user_name)
    .expect("Failed to read line");

    let mut user_pass = String::new();
    println!("Enter your password: ");
    io::stdin().read_line(&mut user_pass)
    .expect("Failed to read line");

    if user_name.trim() == user_check_val && user_pass.trim() == pass_check_val {
        true
    }

    else {
        println!(" {} {} \n {} {}",user_name,user_check_val,user_pass,pass_check_val);
        false
    }
}

// fn generate_report(products: &Vec<i32>) {
//     if products.len() <=0 {
//         println!("Products List is Empty :<");
//         return;
//     }

//     for item in products.iter() {
//         println!("{}",item);
//     }
// }
