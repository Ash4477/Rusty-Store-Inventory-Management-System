use std::io;

// struct Product {
//     name : String,
//     description: String,
//     price: f64,
//     quantity: i32,
// }

// impl Product {
//     fn edit_name(&mut self, new_name: String) {
//         self.name = new_name;
//     }

//     fn edit_price(&mut self, new_price: f64) {
//         self.price = new_price;
//     }

//     fn edit_description(&mut self, new_desc: String) {
//         self.description = new_desc;
//     }

//     fn edit_quantity(&mut self, new_quantity: i32) {
//         self.quantity = new_quantity;
//     }
// }

fn main() {

    // if !admin_check() {
    //     println!("Sorry, wrong credentials :<\nExiting System!");
    //     return;
    // }

    // let mut products : Vec<i32> = Vec::new();

    let mut menu_choice: i32 = 0;

    println!("\tRusty Inventory Management\n----------------------------------------
    => 1: Add Product
    => 2: Edit Product Info
    => 3: Delete Product
    => 4: Exit System
    \tEnter your choice:
    ");

    while menu_choice != 0 {
        menu_choice = integer_input();  
        match menu_choice {
            1 => addProduct(),
            2 => editProduct(),
            3 => deleteProduct(),
            4 => return,
            _ => println!("Choose a valid option!\nEnter:"),
        }
    }

}

fn integer_input() {
    let mut input = String::new();
    let mut parsed_input: Result<i32, _>;
    loop {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        parsed_input= input.trim().parse();

        match parsed_input {
            Ok(num) => {
                break;
            },
            Err(_) => println!("Invalid input!\nPlease enter an integer:"),
        } 
    }

    parsed_input
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



fn addProduct() {}
fn editProduct() {}
fn deleteProduct() {}

fn generate_report(products: &Vec<i32>) {
    if products.len() <=0 {
        println!("Products List is Empty :<");
    }

    else {
        println!("\t      'Inventory Report'\n");
        println!("-------------------");
        for (name, description, price, quantity) in products {
            println!("Name: {}", name);
            println!("Description: {}", description);
            println!("Price: ${:.2}", price);
            println!("Quantity: {}", quantity);
            println!("-------------------");
        }
    }
}
