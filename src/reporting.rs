use crate::inventory::Inventory;

pub trait Reporting {
    fn generate_inventory_report(&self);

    fn generate_sales_report(&self);

    fn generate_purchase_report(&self);
}

impl Reporting for Inventory {
    fn generate_inventory_report(&self) {
        println!("\n--- Inventory Report ---");

        if self.products.is_empty() {
            println!("No products in inventory.");
        } else {
            println!(
                "{:<20} {:<10} {:<10} {:<10}",
                "Product Name", "Quantity", "Price", "Description"
            );
            println!("{:-<60}", "");

            for product in &self.products {
                println!(
                    "{:<20} {:<10} ${:<9.2} {:<10}",
                    product.name, product.quantity, product.price, product.description
                );
            }
        }
    }

    fn generate_sales_report(&self) {
        println!("\n--- Sales Report ---");
        if self.sales.is_empty() {
            println!("No sales recorded.");
        } else {
            println!(
                "{:<20} {:<10} {:<10} {:<10} {:<10}",
                "Product Name", "Quantity", "Sale Price", "Total Sale", "Profit"
            );
            println!("{:-<70}", "");
            for sale in &self.sales {
                let total_sale = sale.sale_price * sale.quantity_sold as f64;
                println!(
                    "{:<20} {:<10} ${:<9.2} ${:<9.2} ${:<9.2}",
                    sale.product_name, sale.quantity_sold, sale.sale_price, total_sale, sale.profit
                );
            }
        }
    }

    fn generate_purchase_report(&self) {
        println!("\n--- Purchase Report ---");
        if self.purchases.is_empty() {
            println!("No purchases recorded.");
        } else {
            println!(
                "{:<20} {:<10} {:<15} {:<10}",
                "Product Name", "Quantity", "Purchase Price", "Total Cost"
            );
            println!("{:-<60}", "");
            for purchase in &self.purchases {
                println!(
                    "{:<20} {:<10} ${:<14.2} ${:<9.2}",
                    purchase.product_name,
                    purchase.quantity_purchased,
                    purchase.purchase_price,
                    purchase.total_cost
                );
            }
        }
    }
}
