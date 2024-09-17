# Rusty Store Inventory Management System

## Overview

The Rusty Store Inventory Management System is a simple yet powerful tool designed to manage the inventory, sales, and purchases of a small retail store. It enables store managers to efficiently track products, record sales and purchases, and generate detailed reports on the store's activities.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Running the Application](#running-the-application)
- [Authentication](#authentication)
- [Main Menu](#main-menu)
- [Inventory Management](#inventory-management)
- [Purchase & Sales Management](#purchase--sales-management)
- [Reporting](#reporting)
- [Project Structure](#project-structure)
- [Contributions](#contributions)

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

## Main Menu
Once logged in, you will be presented with a text-based interface offering the following options:

- **[1] Inventory Management**
  - View, add, remove, or update product information.
- **[2] Purchase Management**
  - Record new purchases and automatically update inventory levels.
- **[3] Sales Management**
  - Track and log sales, adjusting the inventory accordingly.
- **[4] Reporting**
  - Generate reports for inventory status, purchases, and sales summaries.
- **[5] Logout**
  - Use the number keys to select options from the menu.

## Inventory Management
- **Add a new product**:
  - Enter product name, quantity, and price.
- **Update product**:
  - Modify the quantity or price of an existing product.
- **Remove product**:
  - Delete a product from the inventory.
- **View inventory**:
  - Displays a list of all products with their stock levels.

## Purchase & Sales Management
- **Record a new purchase**:
  - Add stock to the inventory by recording new purchases.
- **Track a sale**:
  - Deduct stock from the inventory and record the sale information.

## Reporting
- Generate a report summarizing current inventory levels and recent sales.
- Reports can be viewed within the application or exported to a file.

## Project Structure

- **`src/main.rs`**: The entry point of the application where the inventory, sales, and purchase operations are demonstrated.
- **`src/inventory.rs`**: Contains the `Product` struct and the `Inventory` struct, along with methods for managing the inventory.
- **`src/sales.rs`**: Contains the `Sale` struct and methods related to recording and reporting sales transactions.
- **`src/purchases.rs`**: Contains the `Purchase` struct and methods related to recording and reporting purchase transactions.
- **`src/reporting.rs`**: Contains the `Reporting` trait and methods for generating reports on inventory, sales, and purchases.
- **`modules/auth.rs`**: Handles user authentication and account management.

## Contributions
Feel free to fork the project and submit pull requests for any new features or improvements.
