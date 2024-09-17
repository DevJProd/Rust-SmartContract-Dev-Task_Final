# Rusty Store Inventory Management System

## Overview

The Rusty Store Inventory Management System is a simple yet powerful tool designed to manage the inventory, sales, and purchases of a small retail store. It enables store managers to efficiently track products, record sales and purchases, and generate detailed reports on the store's activities.

## Features

- **Authentication System** : Users are required to log in before accessing the inventory features.
- **Inventory Management** : Add, remove, and update products in inventory.
- **Purchase & Sales Management** : Track purchases and sales, update stock levels accordingly.
- **Reporting** : Generate detailed reports of sales and inventory status.
- **TUI (Text User Interface)**: Navigate the system using keyboard shortcuts within a text-based interface.

## Requirements
To run this project locally, ensure you have the following installed:

- Rust (1.60+ recommended)
- Cargo (Rust package manager)
- Git (optional, if cloning from repository)

## Installation
1. **Clone the repository**:
```bash
git clone https://github.com/YourUsername/rust-inventory-system.git
```
Or download the project as a ZIP and extract it.


2. **Navigate to the project directory**:
```bash
cd rust-inventory-system
```

3. **Build the project: This will compile the Rust code and build the executable.**
```bash
cargo build --release
```
4. **Run the tests (Optional): The project includes unit tests to verify the functionality of the different modules (authentication, inventory, purchases, sales).**

```bash
cargo test
```

## Running the Application
After building the project, run the application using:

```bash
cargo run --release
```

## Authentication
When you start the system, you will be prompted to log in. For first-time users, you will need to create a new account by following the prompts.
```bash
Enter username: admin
Enter password: password123
```

## Project Structure

- **`src/main.rs`**: The entry point of the application where the inventory, sales, and purchase operations are demonstrated.
- **`src/inventory.rs`**: Contains the `Product` struct and the `Inventory` struct, along with methods for managing the inventory.
- **`src/sales.rs`**: Contains the `Sale` struct and methods related to recording and reporting sales transactions.
- **`src/purchases.rs`**: Contains the `Purchase` struct and methods related to recording and reporting purchase transactions.
- **`src/reporting.rs`**: Contains the `Reporting` trait and methods for generating reports on inventory, sales, and purchases.
- **`modules/auth.rs`**: Handles user authentication and account management.

## Contributions
Feel free to fork the project and submit pull requests for any new features or improvements.
