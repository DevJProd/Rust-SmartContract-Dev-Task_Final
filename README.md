# Rusty Store Inventory Management System

## Overview

The Rusty Store Inventory Management System is a simple yet powerful tool designed to manage the inventory, sales, and purchases of a small retail store. It enables store managers to efficiently track products, record sales and purchases, and generate detailed reports on the store's activities.

## Features

- **Inventory Management**: Add, edit, and delete products from the inventory. Each product has a name, description, price, and quantity.
- **Sales Management**: Record sales transactions, including the product sold, the quantity sold, and the sale price. The system calculates and displays the total sales and profit from each transaction.
- **Purchase Management**: Record purchase transactions, including the product purchased, the quantity purchased, and the purchase price. The system calculates and displays the total cost of each purchase.
- **Reporting**: Generate and display reports on the store's inventory, sales, and purchase history in a user-friendly, well-structured format.

## Project Structure

- **`main.rs`**: The entry point of the application where the inventory, sales, and purchase operations are demonstrated.
- **`inventory.rs`**: Contains the `Product` struct and the `Inventory` struct, along with methods for managing the inventory.
- **`sales.rs`**: Contains the `Sale` struct and methods related to recording and reporting sales transactions.
- **`purchases.rs`**: Contains the `Purchase` struct and methods related to recording and reporting purchase transactions.
- **`reporting.rs`**: Contains the `Reporting` trait and methods for generating reports on inventory, sales, and purchases.

## Setup Instructions

### Prerequisites

Ensure you have [Rust installed](https://www.rust-lang.org/tools/install) on your local machine.

### Cloning the Repository

To clone the repository and set up the project on your local machine, run the following commands:

```bash
git clone https://github.com/yourusername/rusty-store-inventory.git
cd rusty-store-inventory

## Building and Running the Program

To build and run the program, execute:

```bash
cargo run
