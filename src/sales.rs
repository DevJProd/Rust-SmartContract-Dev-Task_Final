use crate::inventory::{Inventory, InventoryError};

pub struct Sale {

    pub product_name: String,
    pub quantity_sold: u32,
    pub sale_price: f64,
    pub total_sale: f64,
    pub profit: f64,
}
#[allow(dead_code)]
pub trait SalesManagement {
    
    fn record_sale(&mut self, product_name: &str, quantity_sold: u32, sale_price: f64) -> Result<(), InventoryError>;
    fn total_sales(&self) -> f64;
    fn total_profit(&self) -> f64;
}

impl SalesManagement for Inventory {

    fn record_sale(&mut self, product_name: &str, quantity_sold: u32, sale_price: f64) -> Result<(), InventoryError> {
        if quantity_sold == 0 || sale_price <= 0.0 {
            return Err(InventoryError::InvalidInput("Quantity sold or sale price cannot be zero or negative".to_string()));
        }

        let product = self.products.iter_mut().find(|p| p.name == product_name);

        match product {
            Some(p) => {
                if p.quantity < quantity_sold {
                    return Err(InventoryError::OutOfStock(product_name.to_string()));
                }
                p.quantity -= quantity_sold;

                let total_sale = sale_price * quantity_sold as f64;
                let profit = total_sale - (p.price * quantity_sold as f64); // Assuming cost price is the same as product price
                
                let sale = Sale {
                    product_name: product_name.to_string(),
                    quantity_sold,
                    sale_price,
                    total_sale,
                    profit,
                };
                self.sales.push(sale);
                Ok(())
            },
            None => Err(InventoryError::ProductNotFound(product_name.to_string())),
        }
    }

    fn total_sales(&self) -> f64 {
        self.sales.iter().map(|sale| sale.total_sale as f64).sum()
    }

    fn total_profit(&self) -> f64 {
        self.sales.iter().map(|sale| sale.profit).sum()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{Inventory, InventoryManagement};

    #[test]
    fn test_record_sale_success() {
        let mut inventory = Inventory::new();
        match inventory.add_product("Laptop".to_string(), "High-performance laptop".to_string(), 1000.0, 10){

            Ok(()) => println!("Product added successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }

        // Record a sale
        let result = inventory.record_sale("Laptop", 2, 1200.0);

        // Check result
        assert!(result.is_ok());

        // Verify the sale details
        let sale = inventory.sales.last().unwrap();
        assert_eq!(sale.product_name, "Laptop");
        assert_eq!(sale.quantity_sold, 2);
        assert_eq!(sale.sale_price, 1200.0);
        assert_eq!(sale.total_sale, 2400.0); // 1200.0 * 2
        assert_eq!(sale.profit, 2400.0 - (1000.0 * 2 as f64)); // Sale price - Cost price
    }

    #[test]
    fn test_record_sale_out_of_stock() {
        let mut inventory = Inventory::new();
        match inventory.add_product("Smartphone".to_string(), "Latest smartphone".to_string(), 800.0, 1){
            Ok(()) => println!("Product removed successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }

        // Try to record a sale with more quantity than available
        let result = inventory.record_sale("Smartphone", 2, 850.0);

        // Check result
        assert_eq!(result, Err(InventoryError::OutOfStock("Smartphone".to_string())));
    }

    #[test]
    fn test_record_sale_product_not_found() {
        let mut inventory = Inventory::new();

        // Try to record a sale for a non-existing product
        let result = inventory.record_sale("NonExistingProduct", 1, 100.0);

        // Check result
        assert_eq!(result, Err(InventoryError::ProductNotFound("NonExistingProduct".to_string())));
    }

    #[test]
    fn test_total_sales() {
        let mut inventory = Inventory::new();
        match inventory.add_product("Laptop".to_string(), "High-performance laptop".to_string(), 1000.0, 10){
            Ok(()) => println!("Product added successfully."),
            Err(e) => eprintln!("Failed to add product: {}", e),
        }
        match inventory.add_product("Smartphone".to_string(), "Latest smartphone".to_string(), 800.0, 5){
            Ok(()) => println!("Product added successfully."),
            Err(e) => eprintln!("Failed to add product: {}", e),
        }
        inventory.record_sale("Laptop", 2, 1200.0).unwrap();
        inventory.record_sale("Smartphone", 3, 850.0).unwrap();

        // Calculate total sales
        let total_sales = inventory.total_sales();
        assert_eq!(total_sales, 2400.0 + 2550.0); // 1200.0 * 2 + 850.0 * 3
    }

    #[test]
    fn test_total_profit() {
        let mut inventory = Inventory::new();
        match inventory.add_product("Laptop".to_string(), "High-performance laptop".to_string(), 1000.0, 10) {
            Ok(()) => println!("Product removed successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }

        match inventory.add_product("Smartphone".to_string(), "Latest smartphone".to_string(), 800.0, 5) {
                Ok(()) => println!("Product removed successfully."),
                Err(e) => eprintln!("Failed to remove product: {}", e),
            }
        
        inventory.record_sale("Laptop", 2, 1200.0).unwrap();
        inventory.record_sale("Smartphone", 3, 850.0).unwrap();

        // Calculate total profit
        let total_profit = inventory.total_profit();
        assert_eq!(total_profit, (2400.0 - 2000.0) + (2550.0 - 2400.0)); // Total Sale - Cost Price
    }
}
