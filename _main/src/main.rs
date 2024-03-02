use std::io;

struct Product {
    name : String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Product {
    fn edit_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn edit_price(&mut self, new_price: f64) {
        self.price = new_price;
    }

    fn edit_description(&mut self, new_desc: String) {
        self.description = new_desc;
    }

    fn edit_quantity(&mut self, new_quantity: i32) {
        self.quantity = new_quantity;
    }
}

fn main() {

    if !admin_check() {
        println!("Sorry, wrong credentials :<\nExiting System!");
        return;
    }

    let mut products : Vec<i32> = Vec::new();

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
