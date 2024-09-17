use crate::auth::{Auth, AuthError};
use crate::inventory::{Inventory, InventoryManagement, InventoryError};
use crate::reporting::Reporting;
use crate::sales::SalesManagement;
use crate::purchases::PurchaseManagement;
use std::io::{self, Write};
use thiserror::Error; // You can use the `thiserror` crate for easier error handling

#[derive(Debug, Error)]
pub enum TuiError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Inventory Error: {0}")]
    Inventory(#[from] InventoryError), // Assuming `InventoryError` exists in inventory.rs

    #[error("Authentication Error: {0}")]
    Auth(#[from] AuthError), // If you want to propagate authentication errors too
}

pub struct Tui {
    auth_system: Auth,
}

impl Tui {
    pub fn new(auth_system: Auth) -> Self {
        Tui { auth_system }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        let mut authenticated = false;

        while !authenticated {
            let username = self.prompt_input("Username: ")?;
            let password = self.prompt_input("Password: ")?;

            match self.auth_system.authenticate(&username, &password) {
                Ok(role) => {
                    authenticated = true;
                    println!("Authentication successful! You are logged in as {:?}", role);
                }
                Err(AuthError::InvalidCredentials(_)) => {
                    println!("Invalid credentials, please try again.");
                }
            }
        }

        self.show_main_menu()
    }

    fn prompt_input(&self, prompt: &str) -> Result<String, io::Error> {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn show_main_menu(&mut self) -> Result<(), io::Error> {
        let mut inventory = Inventory::new(); // Assuming an initialized inventory
    
        loop {
            println!("Welcome to the Store Management System");
            println!("1. Manage Inventory");
            println!("2. Record Sale");
            println!("3. Record Purchase");
            println!("4. Generate Report");
            println!("5. Exit");
    
            let choice = self.prompt_input("Select an option: ")?;
            match choice.as_str() {
                "1" => self.manage_inventory(&mut inventory)?,
                "2" => self.record_sale(&mut inventory)?,
                "3" => self.record_purchase(&mut inventory)?,
                "4" => self.generate_report(&inventory)?,
                "5" => {
                    println!("Exiting...");
                    break;
                }
                _ => println!("Invalid choice, please try again."),
            }
        }
        Ok(())
    }

    fn manage_inventory(&mut self, inventory: &mut Inventory) -> Result<(), io::Error> {
        println!("--- Manage Inventory ---");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. List Products");
        println!("5. Back to Main Menu");
    
        let choice = self.prompt_input("Select an option: ")?;
        match choice.as_str() {
            "1" => {
                let name = self.prompt_input("Product Name: ")?;
                let description = self.prompt_input("Product Description: ")?;
                let price: f64 = self.prompt_input("Product Price: ")?.parse().unwrap();
                let quantity: u32 = self.prompt_input("Product Quantity: ")?.parse().unwrap();

                match inventory.add_product(name, description, price, quantity){
                    Ok(()) => println!("Product added successfully."),
                    Err(e) => eprintln!("Failed to add product: {}", e),
                }
            },
            "2" => {
                let product_name = self.prompt_input("Product Name to Edit: ")?;
                let new_name = self.prompt_input("New name: ").ok();
                let new_description = self.prompt_input("New description: ").ok();
                let new_price = self.prompt_input("New Price: ")?.parse::<f64>().ok();
                let new_quantity = self.prompt_input("New Quantity: ")?.parse::<u32>().ok();

                match inventory.edit_product(
                    &product_name,
                    new_name,
                    new_description,
                    new_price,
                    new_quantity
                ){
                    Ok(()) => println!("Product removed successfully."),
                    Err(e) => eprintln!("Failed to remove product: {}", e),
                }
            },
            "3" => {
                let product_name = self.prompt_input("Product Name to Delete: ")?;

                match inventory.delete_product(&product_name){
                    Ok(()) => println!("Product removed successfully."),
                    Err(e) => eprintln!("Failed to remove product: {}", e),
                }
            },
            "4" => inventory.list_products(),
            _ => println!("Invalid choice, returning to main menu."),
        }
        Ok(())
    }

    fn record_sale(&mut self, inventory: &mut Inventory) -> Result<(), io::Error> {
        let product_name = self.prompt_input("Product Name: ")?;
        let quantity: u32 = self.prompt_input("Quantity Sold: ")?.parse().unwrap();
        let sale_price: f64 = self.prompt_input("Sale Price: ")?.parse().unwrap();
        
        match inventory.record_sale(&product_name, quantity, sale_price) {
            Ok(_) => println!("Sale recorded successfully."),
            Err(e) => println!("Error recording sale: {}", e),
        }
        Ok(())
    }
    
    fn record_purchase(&mut self, inventory: &mut Inventory) -> Result<(), io::Error> {
        let product_name = self.prompt_input("Product Name: ")?;
        let quantity: u32 = self.prompt_input("Quantity Purchased: ")?.parse().unwrap();
        let purchase_price: f64 = self.prompt_input("Purchase Price: ")?.parse().unwrap();
        
        match inventory.record_purchase(&product_name, quantity, purchase_price) {
            Ok(_) => println!("Purchase recorded successfully."),
            Err(e) => println!("Error recording purchase: {}", e),
        }
        Ok(())
    }

    fn generate_report(&self, inventory: &Inventory) -> Result<(), io::Error> {
        println!("--- Generate Report ---");
        println!("1. Inventory Report");
        println!("2. Sales Report");
        println!("3. Purchase Report");
        
        let choice = self.prompt_input("Select an option: ")?;
        match choice.as_str() {
            "1" => inventory.generate_inventory_report(),
            "2" => inventory.generate_sales_report(),
            "3" => inventory.generate_purchase_report(),
            _ => println!("Invalid choice, returning to main menu."),
        }
        Ok(())
    }
    

}
