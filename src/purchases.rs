use crate::inventory::{Product, Inventory, InventoryError};


#[allow(dead_code)]
pub trait PurchaseManagement {
    fn record_purchase(
        &mut self,
        product_name: &str,
        quantity_purchased: u32,
        purchase_price: f64,
    ) -> Result<(), InventoryError>;
    fn total_purchase_cost(&self) -> f64;
}

pub struct Purchase {
    pub product_name: String,
    pub quantity_purchased: u32,
    pub purchase_price: f64,
    pub total_cost: f64,
}

impl PurchaseManagement for Inventory {
    fn record_purchase(
        &mut self,
        product_name: &str,
        quantity_purchased: u32,
        purchase_price: f64,
    ) -> Result<(), InventoryError> {
        if quantity_purchased == 0 || purchase_price <= 0.0 {
            return Err(InventoryError::InvalidInput(
                "Quantity purchased or purchase price cannot be zero or negative".to_string(),
            ));
        }

        let total_cost = purchase_price * quantity_purchased as f64;
        let product = self.products.iter_mut().find(|p| p.name == product_name);

        match product {
            Some(p) => {
                p.quantity += quantity_purchased;
                let purchase = Purchase {
                    product_name: product_name.to_string(),
                    quantity_purchased,
                    purchase_price,
                    total_cost,
                };
                self.purchases.push(purchase);
                Ok(())
            }
            None => {
                let new_product = Product {
                    name: product_name.to_string(),
                    description: "Newly purchased product".to_string(),
                    price: purchase_price,
                    quantity: quantity_purchased,
                };
                self.products.push(new_product);
                let purchase = Purchase {
                    product_name: product_name.to_string(),
                    quantity_purchased,
                    purchase_price,
                    total_cost,
                };
                self.purchases.push(purchase);
                Ok(())
            }
        }
    }

    fn total_purchase_cost(&self) -> f64 {
        self.purchases
            .iter()
            .map(|purchase| purchase.total_cost)
            .sum()
    }

}

// Tests for PurchaseManagement
#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{Inventory, InventoryError, InventoryManagement};

    #[test]
    fn test_record_purchase_success_existing_product() {
        let mut inventory = Inventory::new();
        match inventory.add_product(
            "Laptop".to_string(),
            "High-performance laptop".to_string(),
            1000.0,
            10,
        ){
            Ok(()) => println!("Product removed successfully."),
            Err(e) => eprintln!("Failed to remove product: {}", e),
        }

        let result = inventory.record_purchase("Laptop", 5, 950.0);
        assert!(result.is_ok());

        let product = inventory.products.iter().find(|p| p.name == "Laptop");
        assert!(product.is_some());
        let product = product.unwrap();
        assert_eq!(product.quantity, 15); // 10 existing + 5 purchased

        let purchase = inventory.purchases.iter().find(|p| p.product_name == "Laptop");
        assert!(purchase.is_some());
        let purchase = purchase.unwrap();
        assert_eq!(purchase.quantity_purchased, 5);
        assert_eq!(purchase.purchase_price, 950.0);
        assert_eq!(purchase.total_cost, 4750.0); // 5 * 950.0
    }

    #[test]
    fn test_record_purchase_success_new_product() {
        let mut inventory = Inventory::new();

        let result = inventory.record_purchase("Smartphone", 10, 500.0);
        assert!(result.is_ok());

        let product = inventory.products.iter().find(|p| p.name == "Smartphone");
        assert!(product.is_some());
        let product = product.unwrap();
        assert_eq!(product.quantity, 10);
        assert_eq!(product.price, 500.0);

        let purchase = inventory.purchases.iter().find(|p| p.product_name == "Smartphone");
        assert!(purchase.is_some());
        let purchase = purchase.unwrap();
        assert_eq!(purchase.quantity_purchased, 10);
        assert_eq!(purchase.purchase_price, 500.0);
        assert_eq!(purchase.total_cost, 5000.0); // 10 * 500.0
    }

    #[test]
    fn test_record_purchase_invalid_quantity() {
        let mut inventory = Inventory::new();

        let result = inventory.record_purchase("Tablet", 0, 300.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), InventoryError::InvalidInput("Quantity purchased or purchase price cannot be zero or negative".to_string()));
    }

    #[test]
    fn test_record_purchase_invalid_price() {
        let mut inventory = Inventory::new();

        let result = inventory.record_purchase("Headphones", 5, -50.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), InventoryError::InvalidInput("Quantity purchased or purchase price cannot be zero or negative".to_string()));
    }

    #[test]
    fn test_total_purchase_cost() {
        let mut inventory = Inventory::new();
        inventory.record_purchase("Monitor", 2, 200.0).unwrap();
        inventory.record_purchase("Keyboard", 3, 50.0).unwrap();

        let total_cost = inventory.total_purchase_cost();
        assert_eq!(total_cost, (2 as f64) * 200.0 + (3 as f64) * 50.0); // 400.0 + 150.0
    }
}