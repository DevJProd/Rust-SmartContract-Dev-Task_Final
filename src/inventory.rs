use crate::purchases::Purchase;
use crate::sales::Sale;
use std::fmt;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum InventoryError {
    ProductNotFound(String),
    InvalidQuantity(u32),
    InvalidPrice(f64),
    OutOfStock(String),
    InvalidInput(String),
    PurchaseFailed(String),
    SaleFailed(String),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::ProductNotFound(name) => write!(f, "Product '{}' not found", name),
            InventoryError::InvalidQuantity(qty) => write!(f, "Invalid quantity: {}", qty),
            InventoryError::InvalidPrice(price) => write!(f, "Invalid price: {}", price),
            InventoryError::OutOfStock(reason) => write!(f, "Purchase failed: {}", reason),
            InventoryError::InvalidInput(reason) => write!(f, "Purchase failed: {}", reason),
            InventoryError::PurchaseFailed(reason) => write!(f, "Purchase failed: {}", reason),
            InventoryError::SaleFailed(reason) => write!(f, "Sale failed: {}", reason),
        }
    }
}

impl std::error::Error for InventoryError {}

pub trait InventoryManagement {
    fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32) -> Result<(), InventoryError>;
    fn edit_product(
        &mut self,
        product_name: &str,
        new_name: Option<String>,
        new_description: Option<String>,
        new_price: Option<f64>,
        new_quantity: Option<u32>,
    ) -> Result<(), InventoryError>;
    fn delete_product(&mut self, product_name: &str) -> Result<(), InventoryError>;
    fn list_products(&self);
}

pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

pub struct Inventory {
    pub products: Vec<Product>,
    pub sales: Vec<Sale>,
    pub purchases: Vec<Purchase>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            products: Vec::new(),
            sales: Vec::new(),
            purchases: Vec::new(),
        }
    }
    /*
        pub fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32) -> Result<(), InventoryError> {
            if price <= 0.0 {
                return Err(InventoryError::InvalidPrice(price));
            }
            if quantity == 0 {
                return Err(InventoryError::InvalidQuantity(quantity));
            }

            let product = Product {
                name,
                description,
                price,
                quantity,
            };
            self.products.push(product);
            Ok(())
        }

        pub fn update_product(&mut self, name: &str, price: Option<f64>, quantity: Option<u32>) -> Result<(), InventoryError> {
            let product = self.products.iter_mut().find(|p| p.name == name);

            match product {
                Some(p) => {
                    if let Some(new_price) = price {
                        if new_price <= 0.0 {
                            return Err(InventoryError::InvalidPrice(new_price));
                        }
                        p.price = new_price;
                    }
                    if let Some(new_quantity) = quantity {
                        if new_quantity == 0 {
                            return Err(InventoryError::InvalidQuantity(new_quantity));
                        }
                        p.quantity = new_quantity;
                    }
                    Ok(())
                },
                None => Err(InventoryError::ProductNotFound(name.to_string())),
            }
        }

        pub fn remove_product(&mut self, name: &str) -> Result<(), InventoryError> {
            let index = self.products.iter().position(|p| p.name == name);

            match index {
                Some(i) => {
                    self.products.remove(i);
                    Ok(())
                },
                None => Err(InventoryError::ProductNotFound(name.to_string())),
            }
        }
    */
}

impl InventoryManagement for Inventory {
    fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32) -> Result<(), InventoryError> {

        if price <= 0.0 {
            return Err(InventoryError::InvalidPrice(price));
        }
        if quantity == 0 {
            return Err(InventoryError::InvalidQuantity(quantity));
        }

        let product = Product {
            name,
            description,
            price,
            quantity,
        };
        self.products.push(product);

        println!("Product added successfully.");
        Ok(())
    }

    fn edit_product(
        &mut self,
        product_name: &str,
        new_name: Option<String>,
        new_description: Option<String>,
        new_price: Option<f64>,
        new_quantity: Option<u32>,
    ) -> Result<(), InventoryError> {
        for product in &mut self.products {
            if product.name == product_name {
                if let Some(name) = new_name {
                    product.name = name;
                }
                if let Some(description) = new_description {
                    product.description = description;
                }
                if let Some(price) = new_price {
                    product.price = price;
                }
                if let Some(quantity) = new_quantity {
                    product.quantity = quantity;
                }
                println!("Product updated successfully.");
                return Ok(())
            }
        }
        return Err(InventoryError::ProductNotFound(product_name.to_string()));
    }

    fn delete_product(&mut self, product_name: &str) -> Result<(), InventoryError> {
        if let Some(pos) = self.products.iter().position(|p| p.name == product_name) {
            self.products.remove(pos);
            println!("Product deleted successfully.");
            Ok(())
        } else {
            return Err(InventoryError::ProductNotFound(product_name.to_string()));
        }
    }

    fn list_products(&self) {
        for product in &self.products {
            println!(
                "Name: {}, Description: {}, Price: ${}, Quantity: {}",
                product.name, product.description, product.price, product.quantity
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_product() {
        let mut inventory = Inventory::new();
        match inventory.add_product(
            "Laptop".to_string(),
            "A high-performance laptop".to_string(),
            1200.0,
            10,
        ) {
            Ok(()) => println!("Product added successfully."),
            Err(e) => eprintln!("Failed to add product: {}", e),
        }

        assert_eq!(inventory.products.len(), 1);
        let product = &inventory.products[0];
        assert_eq!(product.name, "Laptop");
        assert_eq!(product.description, "A high-performance laptop");
        assert_eq!(product.price, 1200.0);
        assert_eq!(product.quantity, 10);
    }

    #[test]
    fn test_edit_product() {
        let mut inventory = Inventory::new();
        match inventory.add_product(
            "Laptop".to_string(),
            "A high-performance laptop".to_string(),
            1200.0,
            10,
        ) {
            Ok(()) => println!("Product added successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }

        // Edit product details
        match inventory.edit_product(
            "Laptop",
            Some("Gaming Laptop".to_string()),
            Some("A high-end gaming laptop".to_string()),
            Some(1500.0),
            Some(5),
        ) {
            Ok(()) => println!("Product edited successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }

        let product = &inventory.products[0];
        assert_eq!(product.name, "Gaming Laptop");
        assert_eq!(product.description, "A high-end gaming laptop");
        assert_eq!(product.price, 1500.0);
        assert_eq!(product.quantity, 5);
    }

    #[test]
    fn test_delete_product() {
        let mut inventory = Inventory::new();
        match inventory.add_product(
            "Laptop".to_string(),
            "A high-performance laptop".to_string(),
            1200.0,
            10,
        ) {
            Ok(()) => println!("Product added successfully."),
            Err(e) => eprintln!("Failed to add product: {}", e),
        }

        assert_eq!(inventory.products.len(), 1);

        // Delete the product
        match inventory.delete_product("Laptop") {
            Ok(()) => println!("Product removed successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }
        assert_eq!(inventory.products.len(), 0);
    }

    #[test]
    fn test_list_products() {
        let mut inventory = Inventory::new();
        match inventory.add_product(
            "Laptop".to_string(),
            "A high-performance laptop".to_string(),
            1200.0,
            10,
        ){
            Ok(()) => println!("Product removed successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }
        match inventory.add_product(
            "Phone".to_string(),
            "A latest smartphone".to_string(),
            800.0,
            20,
        ){
            Ok(()) => println!("Product removed successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }
    }
}