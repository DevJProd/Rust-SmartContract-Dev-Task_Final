mod auth;
mod tui;
mod inventory;
mod purchases;
mod sales;
mod reporting;

use crate::auth::Auth;
use crate::tui::Tui;

fn main() {
    let mut auth_system = Auth::new();
    auth_system.users.insert(
        "admin".to_string(),
        crate::auth::User {
            username: "admin".to_string(),
            password: "password".to_string(),
            role: crate::auth::UserRole::Admin,
        },
    );

    let mut tui = Tui::new(auth_system);
    if let Err(e) = tui.run() {
        eprintln!("Error: {}", e);
    }


}
/*
mod inventory;
mod purchases;
mod reporting;
mod sales;

use inventory::Inventory;
use purchases::PurchaseManagement;
use reporting::Reporting;
use sales::SalesManagement;

use crate::inventory::InventoryManagement;

fn main() {
    let mut inventory = Inventory::new();

    // Add products
    inventory.add_product(
        "Laptop".to_string(),
        "High-performance laptop".to_string(),
        1200.0,
        10,
    );

    inventory.edit_product(
        "Laptop",
        Some("Updated_Lap".to_string()),
        Some("Updated High-performance laptop".to_string()),
        Some(1000.0),
        Some(5),
    );

    inventory.add_product(
        "Smartphone".to_string(),
        "Latest smartphone model".to_string(),
        800.0,
        20,
    );

    // inventory.delete_product("Smartphone");

    // List products
    println!("Listing initial products...");
    inventory.list_products();

    // Record a purchase
    println!("\nRecording a purchase...");
    inventory.record_purchase("Laptop", 5, 1150.0).unwrap();

    // Record a sale
    println!("\nRecording a sale...");
    inventory.record_sale("Smartphone", 3, 850.0).unwrap();

    println!("{}", inventory.total_purchase_cost());
    println!("{}", inventory.total_sales());
    println!("{}", inventory.total_profit());

    // Generate reports
    println!("\nGenerating reports...");
    inventory.generate_inventory_report();
    inventory.generate_sales_report();
    inventory.generate_purchase_report();
}
*/