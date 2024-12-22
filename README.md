# Car Wash Management System

The Car Wash Management System is a smart and efficient system for managing car wash businesses. It provides features for registering customers, tracking vehicles, scheduling car washes, recording payments, managing services, tracking expenses, and generating reports.

---

## Features

1. **Customer Management**
   - Add, update, and delete customer information.
   - Retrieve customer details by ID.

2. **Vehicle Management**
   - Register and manage vehicles.
   - Track vehicle details such as make, model, license plate, and owner.

3. **Service Management**
   - Define different car wash services with prices.
   - Update service details as needed.

4. **Car Wash Scheduling**
   - Schedule car wash appointments with assigned staff and services.
   - Update the status of appointments (e.g., "scheduled," "completed," "canceled").

5. **Expense Tracking**
   - Record operational expenses with categories and descriptions.
   - Generate expense reports.

6. **Revenue Management**
   - Record payments for services.
   - Track total revenue.

7. **Reports and Analytics**
   - Generate reports for:
     - Total revenue
     - Total expenses
     - Profit/loss analysis

---

## System Design

The system leverages the following Rust structures and functionalities:

- **Stable Storage**: Used to persistently store customer, vehicle, service, appointment, and financial data.
- **Thread-safe Memory Management**: Ensures data consistency in a concurrent environment.
- **Modular Design**: Simplifies adding new features and functionality.

---

## Data Models

1. **Customer**
   - `id`
   - `name`
   - `contact`
   - `email`
   - `address`

2. **Vehicle**
   - `id`
   - `customer_id`
   - `make`
   - `model`
   - `license_plate`
   - `color`

3. **Service**
   - `id`
   - `name`
   - `description`
   - `price`

4. **Appointment**
   - `id`
   - `vehicle_id`
   - `service_id`
   - `appointment_time`
   - `status`

5. **Expense**
   - `id`
   - `category`
   - `amount`
   - `description`
   - `date`

6. **Payment**
   - `id`
   - `appointment_id`
   - `amount`
   - `payment_method`
   - `status`

---

## API Endpoints

1. **Customer Management**
   - Create customer
   - Retrieve customer by ID
   - List all customers

2. **Vehicle Management**
   - Register vehicle
   - Retrieve vehicles by customer ID

3. **Service Management**
   - Add service
   - Retrieve all services

4. **Car Wash Scheduling**
   - Create appointment
   - Update appointment status
   - Retrieve appointments by vehicle ID

5. **Expense Tracking**
   - Record expense
   - Retrieve all expenses

6. **Revenue Management**
   - Record payment
   - Retrieve total revenue

7. **Analytics**
   - Calculate total revenue
   - Calculate total expenses
   - Generate profit/loss report

---


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown targetz
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```