
write me up an example rust file that has many examples of technical debt, complexity, duplication, etc. (the types of issues that would be detected by qlty smells and structure checks)

12:35 PM
technical_debt_example.rs
Code 
I've created a Rust file packed with various code smells and technical debt patterns that static analysis tools like qlty would flag.

Here's a summary of the issues included:

Structural Issues

God Class: ApplicationManager handles users, products, orders, config, caching, logging, sessions, email, and metrics all in one struct
Long Method: process_user_order does way too much in a single function
Long Parameter List: create_user and get_shipping_cost have too many parameters
Deep Nesting: 5+ levels of nested conditionals in process_user_order
Duplication

find_user and get_user_by_id are nearly identical
find_product and get_product_by_id are nearly identical
Repeated user/product lookup logic inline in process_user_order
Duplicated logging logic
Code Smells

Magic Numbers: 0.08 (tax), 50.0, 5.99, 100, 0.1, 0.15, etc. scattered throughout
Feature Envy: calculate_user_bonus is more interested in User fields than ApplicationManager
Data Clumps: user_name, user_email, user_phone always appear together across multiple methods
Primitive Obsession: Using f64 for money instead of a proper Money type
Message Chains/Law of Demeter Violation: company.department.manager.assistant.contact.email
Inappropriate Intimacy: generate_report reaches deep into all data structures
Dead Code & Speculation

internal_helper_unused and another_unused_method are never called
Speculative Generality: process_data_generic is over-engineered for hypothetical future use
Design Issues

Refused Bequest: SimpleProcessor panics on inherited methods it doesn't want
Parallel Inheritance Hierarchies: Animal/AnimalFood with Dog/DogFood, etc.
Divergent Change: do_everything changes for many unrelated reasons
Global Mutable State: GLOBAL_CONFIG, GLOBAL_CACHE, GLOBAL_COUNTER
Documentation Smells

Comments explaining bad code instead of fixing it
TODO/FIXME/HACK comments indicating known problems





Claude is AI and can make mistakes. Please double-check responses.
// This file demonstrates various code smells and technical debt patterns
// that static analysis tools like qlty would flag

#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

// GOD CLASS: This struct does way too many unrelated things
pub struct ApplicationManager {
    users: Vec<User>,
    products: Vec<Product>,
    orders: Vec<Order>,
    config: HashMap<String, String>,
    db_connection: Option<String>,
    cache: HashMap<String, String>,
    log_level: u8,
    temp_data: Vec<u8>,
    session_tokens: HashMap<String, String>,
    email_queue: Vec<String>,
    metrics: HashMap<String, i64>,
    feature_flags: HashMap<String, bool>,
}

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_active: bool,
    pub role: String,
    pub department: String,
    pub salary: f64,
}

#[derive(Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub category: String,
}

#[derive(Clone)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub total: f64,
    pub status: String,
}

impl ApplicationManager {
    // LONG METHOD: This function is way too long and does too many things
    pub fn process_user_order(&mut self, user_id: i32, product_id: i32, quantity: i32) -> Result<String, String> {
        // Find user - DUPLICATED LOGIC (appears multiple times)
        let mut found_user: Option<User> = None;
        for i in 0..self.users.len() {
            if self.users[i].id == user_id {
                found_user = Some(self.users[i].clone());
                break;
            }
        }
        
        let user = match found_user {
            Some(u) => u,
            None => return Err("User not found".to_string()),
        };
        
        // Check if user is active - MAGIC NUMBER
        if user.is_active == false {
            return Err("User is not active".to_string());
        }
        
        // Find product - DUPLICATED LOGIC
        let mut found_product: Option<Product> = None;
        for i in 0..self.products.len() {
            if self.products[i].id == product_id {
                found_product = Some(self.products[i].clone());
                break;
            }
        }
        
        let product = match found_product {
            Some(p) => p,
            None => return Err("Product not found".to_string()),
        };
        
        // MAGIC NUMBERS everywhere
        if quantity < 1 {
            return Err("Invalid quantity".to_string());
        }
        if quantity > 100 {
            return Err("Quantity too large".to_string());
        }
        
        // Calculate total - PRIMITIVE OBSESSION (should use a Money type)
        let subtotal = product.price * quantity as f64;
        let tax = subtotal * 0.08; // MAGIC NUMBER
        let shipping = if subtotal > 50.0 { 0.0 } else { 5.99 }; // MAGIC NUMBERS
        let discount = if quantity > 10 { subtotal * 0.1 } else { 0.0 }; // MAGIC NUMBERS
        let total = subtotal + tax + shipping - discount;
        
        // DEEP NESTING
        if user.role == "premium" {
            if product.category == "electronics" {
                if quantity >= 5 {
                    if subtotal > 200.0 {
                        if user.department == "sales" {
                            // Apply special discount
                            let special_discount = total * 0.15;
                            let final_total = total - special_discount;
                            
                            // Create order
                            let order = Order {
                                id: self.orders.len() as i32 + 1,
                                user_id: user_id,
                                product_id: product_id,
                                quantity: quantity,
                                total: final_total,
                                status: "pending".to_string(),
                            };
                            self.orders.push(order);
                            
                            // Log - DUPLICATED LOGGING LOGIC
                            println!("[INFO] Order created for user {} - total: {}", user_id, final_total);
                            self.metrics.insert("orders_created".to_string(), 
                                self.metrics.get("orders_created").unwrap_or(&0) + 1);
                            
                            return Ok(format!("Order created with special discount: ${:.2}", final_total));
                        }
                    }
                }
            }
        }
        
        // Create regular order
        let order = Order {
            id: self.orders.len() as i32 + 1,
            user_id: user_id,
            product_id: product_id,
            quantity: quantity,
            total: total,
            status: "pending".to_string(),
        };
        self.orders.push(order);
        
        // Log - DUPLICATED LOGGING LOGIC
        println!("[INFO] Order created for user {} - total: {}", user_id, total);
        self.metrics.insert("orders_created".to_string(), 
            self.metrics.get("orders_created").unwrap_or(&0) + 1);
        
        Ok(format!("Order created: ${:.2}", total))
    }
    
    // DUPLICATED METHOD: Almost identical to get_user_by_id
    pub fn find_user(&self, user_id: i32) -> Option<User> {
        for i in 0..self.users.len() {
            if self.users[i].id == user_id {
                return Some(self.users[i].clone());
            }
        }
        None
    }
    
    // DUPLICATED METHOD: Almost identical to find_user
    pub fn get_user_by_id(&self, id: i32) -> Option<User> {
        for i in 0..self.users.len() {
            if self.users[i].id == id {
                return Some(self.users[i].clone());
            }
        }
        None
    }
    
    // DUPLICATED METHOD: Same pattern for products
    pub fn find_product(&self, product_id: i32) -> Option<Product> {
        for i in 0..self.products.len() {
            if self.products[i].id == product_id {
                return Some(self.products[i].clone());
            }
        }
        None
    }
    
    // DUPLICATED METHOD: Same pattern for products
    pub fn get_product_by_id(&self, id: i32) -> Option<Product> {
        for i in 0..self.products.len() {
            if self.products[i].id == id {
                return Some(self.products[i].clone());
            }
        }
        None
    }
    
    // FEATURE ENVY: This method is more interested in User than ApplicationManager
    pub fn calculate_user_bonus(&self, user_id: i32) -> f64 {
        let user = self.find_user(user_id);
        match user {
            Some(u) => {
                let base = u.salary * 0.1;
                let tenure_bonus = if u.created_at.starts_with("2020") { 500.0 } 
                    else if u.created_at.starts_with("2021") { 400.0 }
                    else if u.created_at.starts_with("2022") { 300.0 }
                    else { 200.0 };
                let role_bonus = if u.role == "manager" { 1000.0 }
                    else if u.role == "senior" { 500.0 }
                    else if u.role == "junior" { 100.0 }
                    else { 0.0 };
                let dept_bonus = if u.department == "engineering" { 800.0 }
                    else if u.department == "sales" { 600.0 }
                    else if u.department == "marketing" { 400.0 }
                    else { 200.0 };
                base + tenure_bonus + role_bonus + dept_bonus
            }
            None => 0.0
        }
    }
    
    // LONG PARAMETER LIST
    pub fn create_user(
        &mut self,
        name: String,
        email: String,
        age: i32,
        address: String,
        phone: String,
        is_active: bool,
        role: String,
        department: String,
        salary: f64,
    ) -> User {
        let user = User {
            id: self.users.len() as i32 + 1,
            name: name,
            email: email,
            age: age,
            address: address,
            phone: phone,
            created_at: "2024-01-01".to_string(), // HARDCODED VALUE
            updated_at: "2024-01-01".to_string(), // HARDCODED VALUE
            is_active: is_active,
            role: role,
            department: department,
            salary: salary,
        };
        self.users.push(user.clone());
        user
    }
    
    // COMPLEX CONDITIONALS / HIGH CYCLOMATIC COMPLEXITY
    pub fn get_shipping_cost(&self, user_id: i32, product_id: i32, quantity: i32, 
                             destination: &str, express: bool, insured: bool,
                             gift_wrap: bool, fragile: bool) -> f64 {
        let mut cost = 0.0;
        
        let user = self.find_user(user_id);
        let product = self.find_product(product_id);
        
        if let (Some(u), Some(p)) = (user, product) {
            // BASE COST - MAGIC NUMBERS
            if destination == "domestic" {
                cost = 5.99;
            } else if destination == "international" {
                cost = 25.99;
            } else if destination == "express_domestic" {
                cost = 15.99;
            } else if destination == "express_international" {
                cost = 45.99;
            } else {
                cost = 10.0;
            }
            
            // Weight adjustment - MORE MAGIC NUMBERS
            if p.category == "electronics" {
                cost += 3.0;
            } else if p.category == "furniture" {
                cost += 15.0;
            } else if p.category == "books" {
                cost += 1.0;
            } else if p.category == "clothing" {
                cost += 2.0;
            }
            
            // Quantity adjustment
            if quantity > 1 && quantity <= 5 {
                cost += 2.0 * (quantity - 1) as f64;
            } else if quantity > 5 && quantity <= 10 {
                cost += 1.5 * (quantity - 1) as f64;
            } else if quantity > 10 {
                cost += 1.0 * (quantity - 1) as f64;
            }
            
            // Options
            if express { cost += 10.0; }
            if insured { cost += 5.0; }
            if gift_wrap { cost += 3.0; }
            if fragile { cost += 7.0; }
            
            // User discounts
            if u.role == "premium" {
                cost *= 0.8;
            } else if u.role == "gold" {
                cost *= 0.85;
            } else if u.role == "silver" {
                cost *= 0.9;
            }
        }
        
        cost
    }
    
    // DEAD CODE: This method is never called
    fn internal_helper_unused(&self) -> String {
        let x = 42;
        let y = "hello";
        format!("{} {}", x, y)
    }
    
    // DEAD CODE: Another unused method
    fn another_unused_method(&mut self, data: Vec<u8>) {
        self.temp_data = data;
        self.temp_data.clear();
    }
    
    // SPECULATIVE GENERALITY: Over-engineered for future use cases
    pub fn process_data_generic<T, U, V, W>(&self, 
        _input: T, 
        _transformer: fn(T) -> U, 
        _validator: fn(U) -> bool,
        _formatter: fn(U) -> V,
        _finalizer: fn(V) -> W) -> Option<W> {
        // TODO: Implement later
        None
    }
    
    // DATA CLUMPS: These parameters always appear together
    pub fn send_notification(&self, user_name: String, user_email: String, 
                             user_phone: String, message: String) {
        println!("Sending to {} at {} / {}: {}", user_name, user_email, user_phone, message);
    }
    
    pub fn log_activity(&self, user_name: String, user_email: String,
                        user_phone: String, activity: String) {
        println!("User {} ({} / {}) performed: {}", user_name, user_email, user_phone, activity);
    }
    
    pub fn create_audit_record(&self, user_name: String, user_email: String,
                               user_phone: String, action: String, timestamp: String) {
        println!("[{}] {} ({} / {}): {}", timestamp, user_name, user_email, user_phone, action);
    }
    
    // INAPPROPRIATE INTIMACY: Reaches too deep into other structures
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        for user in &self.users {
            report.push_str(&format!("User: {} - {} - {} - {} - {} - {} - {}\n",
                user.name, user.email, user.phone, user.address, 
                user.department, user.role, user.salary));
        }
        
        for product in &self.products {
            report.push_str(&format!("Product: {} - {} - {} - {}\n",
                product.name, product.price, product.category, product.description));
        }
        
        for order in &self.orders {
            report.push_str(&format!("Order: {} - {} - {} - {} - {}\n",
                order.id, order.user_id, order.product_id, order.quantity, order.total));
        }
        
        report
    }
}

// REFUSED BEQUEST: Subtype that doesn't use parent's interface properly
pub trait DataProcessor {
    fn process(&self, data: &str) -> String;
    fn validate(&self, data: &str) -> bool;
    fn transform(&self, data: &str) -> String;
    fn finalize(&self, data: &str) -> String;
}

pub struct SimpleProcessor;

impl DataProcessor for SimpleProcessor {
    fn process(&self, data: &str) -> String {
        data.to_uppercase()
    }
    
    fn validate(&self, _data: &str) -> bool {
        true // LAZY IMPLEMENTATION
    }
    
    fn transform(&self, _data: &str) -> String {
        panic!("Not implemented") // REFUSED BEQUEST
    }
    
    fn finalize(&self, _data: &str) -> String {
        panic!("Not implemented") // REFUSED BEQUEST
    }
}

// PARALLEL INHERITANCE HIERARCHIES
pub trait Animal {
    fn speak(&self) -> String;
}

pub trait AnimalFood {
    fn get_food(&self) -> String;
}

pub struct Dog;
pub struct DogFood;

pub struct Cat;
pub struct CatFood;

pub struct Bird;
pub struct BirdFood;

impl Animal for Dog {
    fn speak(&self) -> String { "Woof".to_string() }
}

impl AnimalFood for DogFood {
    fn get_food(&self) -> String { "Kibble".to_string() }
}

impl Animal for Cat {
    fn speak(&self) -> String { "Meow".to_string() }
}

impl AnimalFood for CatFood {
    fn get_food(&self) -> String { "Fish".to_string() }
}

impl Animal for Bird {
    fn speak(&self) -> String { "Tweet".to_string() }
}

impl AnimalFood for BirdFood {
    fn get_food(&self) -> String { "Seeds".to_string() }
}

// MESSAGE CHAINS / LAW OF DEMETER VIOLATION
pub struct Company {
    pub department: Department,
}

pub struct Department {
    pub manager: Manager,
}

pub struct Manager {
    pub assistant: Assistant,
}

pub struct Assistant {
    pub contact: Contact,
}

pub struct Contact {
    pub email: String,
}

pub fn get_manager_assistant_email(company: &Company) -> String {
    // Violates Law of Demeter - too many dots
    company.department.manager.assistant.contact.email.clone()
}

// COMMENTS EXPLAINING BAD CODE (instead of fixing the code)
pub fn confusing_calculation(a: i32, b: i32, c: i32) -> i32 {
    // First we take a and multiply by 3, then add b squared
    // Then we subtract c times 2, but only if c is positive
    // Finally we add 42 because that's what the business wanted
    // Don't ask me why, I didn't write the original spec
    // TODO: Refactor this someday
    // FIXME: This breaks for negative numbers
    // HACK: Adding 1 to fix off-by-one error
    let x = a * 3 + b * b;
    let y = if c > 0 { x - c * 2 } else { x };
    y + 42 + 1
}

// SHOTGUN SURGERY SETUP: Changes here require changes in many places
pub static mut GLOBAL_CONFIG: Option<HashMap<String, String>> = None;
pub static mut GLOBAL_CACHE: Option<HashMap<String, String>> = None;
pub static mut GLOBAL_COUNTER: i32 = 0;

pub fn init_globals() {
    unsafe {
        GLOBAL_CONFIG = Some(HashMap::new());
        GLOBAL_CACHE = Some(HashMap::new());
        GLOBAL_COUNTER = 0;
    }
}

pub fn use_globals() {
    unsafe {
        if let Some(ref mut config) = GLOBAL_CONFIG {
            config.insert("key".to_string(), "value".to_string());
        }
        GLOBAL_COUNTER += 1;
    }
}

// DIVERGENT CHANGE: This function changes for many different reasons
pub fn do_everything(action: &str, data: &str) -> String {
    match action {
        "parse_json" => format!("Parsed JSON: {}", data),
        "parse_xml" => format!("Parsed XML: {}", data),
        "parse_csv" => format!("Parsed CSV: {}", data),
        "validate_email" => format!("Validated email: {}", data),
        "validate_phone" => format!("Validated phone: {}", data),
        "format_date" => format!("Formatted date: {}", data),
        "format_currency" => format!("Formatted currency: {}", data),
        "encrypt" => format!("Encrypted: {}", data),
        "decrypt" => format!("Decrypted: {}", data),
        "compress" => format!("Compressed: {}", data),
        "decompress" => format!("Decompressed: {}", data),
        _ => "Unknown action".to_string(),
    }
}
