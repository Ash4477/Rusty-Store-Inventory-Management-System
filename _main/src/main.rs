use std::collections::HashMap;
use std::io::{self, Write};

struct Product {
    name : String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Product {
    fn edit_name(&mut self, new_name: String) {
        self.name = new_name.to_uppercase();
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

struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn add_product(&mut self, name: String, description: String, price: f64, quantity: i32) {
        let product = Product { name: name.clone(), description, price, quantity };
        self.products.insert(name.clone(), product);
        println!("Product '{}' added to inventory.", name);
    }

    fn edit_product(&mut self, name: &str, new_description: String, new_price: f64, new_quantity: i32) {
        if let Some(product) = self.products.get_mut(name) {
            product.description = new_description;
            product.price = new_price;
            product.quantity = new_quantity;
            println!("Product '{}' updated.", name);
        } else {
            println!("Product '{}' not found.", name);
        }
    }

    fn delete_product(&mut self, name: &str) {
        if let Some(_) = self.products.remove(name) {
            println!("Product '{}' deleted from inventory.", name);
        } else {
            println!("Product '{}' not found.", name);
        }
    }

    fn generate_report(&self) {
        println!("\t      'Inventory Report'\n");
        println!("-------------------");
        for (name, product) in &self.products {
            println!("Name: {}", name);
            println!("Description: {}", product.description);
            println!("Price: ${:.2}", product.price);
            println!("Quantity: {}", product.quantity);
            println!("-------------------");
        }
    }
}

fn main() {

    // if !admin_check() {
    //     println!("Sorry, wrong credentials :<\nExiting System!");
    //     return;
    // }

    let mut inventory = Inventory { products: HashMap::new() };

    let mut menu_choice: i32;

    println!("\tRusty Inventory Management\n----------------------------------------
    => 1: Add Product
    => 2: Edit Product Info
    => 3: Delete Product
    => 4: Exit System
    ");

    loop {
        menu_choice = get_integer_input();  
        match menu_choice {
            1 => {
                let mut name = String::new();
                let mut price: f64;
                let mut description = String::new();
                let mut quantity: i32;

                println!("Enter Product Name:");
                io::stdin().read_line(&mut name).expect("Failed to read line");

                println!("Enter Price:");
                price = get_float_input();

                println!("Enter Description:");
                io::stdin().read_line(&mut description).expect("Failed to read line");

                println!("Enter Quantity:");
                quantity = get_integer_input();

                inventory.add_product(name.trim().to_uppercase(), description.trim().to_string(), price, quantity);
            },

            // 2 => editProduct(),
            // 3 => deleteProduct(),
            4 => {
                println!("Goodbye :>");
                return;
            },
            _ => println!("Choose a valid option!"),
        }
    }

}

fn get_integer_input() -> i32 {
    loop {
        println!("Enter: ");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter an integer."),
        }
    }
}

fn get_float_input() -> f64 {
    loop {
        println!("Enter: ");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a float value."),
        }
    }
}

// fn admin_check() -> bool {

//     // For the sake of simplicity, I have set admin as login user and password!
//     let user_check_val : String = String::from("admin");
//     let pass_check_val : String = String::from("admin");

//     println!("\n----------------------------------------\n\tRusty Inventory Management\n----------------------------------------");
//     println!("\t      'Login Panel'\n");

//     let mut user_name = String::new();
//     println!("Enter your username: ");
//     io::stdin().read_line(&mut user_name)
//     .expect("Failed to read line");

//     let mut user_pass = String::new();
//     println!("Enter your password: ");
//     io::stdin().read_line(&mut user_pass)
//     .expect("Failed to read line");

//     if user_name.trim() == user_check_val && user_pass.trim() == pass_check_val {
//         true
//     }

//     else {
//         false
//     }
// }




