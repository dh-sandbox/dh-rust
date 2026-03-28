// ============================================================
// CODE SMELL: Dead code / unused functions
// ============================================================

fn unused_helper_one() -> i32 {
    let x = 42;
    x
}

fn unused_helper_two() -> String {
    let s = String::from("never called");
    s
}

fn unused_helper_three() -> bool {
    let b = true;
    b
}

// ============================================================
// CODE SMELL: Duplicated logic (copy-paste programming)
// ============================================================

pub fn process_user_data_v1(name: &str, age: i32, email: &str) -> String {
    let mut result = String::new();
    if name.is_empty() {
        result.push_str("ERROR: name is empty. ");
    }
    if age < 0 {
        result.push_str("ERROR: age is negative. ");
    }
    if age > 150 {
        result.push_str("ERROR: age is too large. ");
    }
    if email.is_empty() {
        result.push_str("ERROR: email is empty. ");
    }
    if !email.contains('@') {
        result.push_str("ERROR: email is invalid. ");
    }
    if result.is_empty() {
        result = format!("User {} (age {}) with email {} is valid", name, age, email);
    }
    result
}

pub fn process_user_data_v2(name: &str, age: i32, email: &str) -> String {
    let mut result = String::new();
    if name.is_empty() {
        result.push_str("ERROR: name is empty. ");
    }
    if age < 0 {
        result.push_str("ERROR: age is negative. ");
    }
    if age > 150 {
        result.push_str("ERROR: age is too large. ");
    }
    if email.is_empty() {
        result.push_str("ERROR: email is empty. ");
    }
    if !email.contains('@') {
        result.push_str("ERROR: email is invalid. ");
    }
    if result.is_empty() {
        result = format!("User {} (age {}) with email {} is valid", name, age, email);
    }
    result
}

pub fn process_user_data_v3(name: &str, age: i32, email: &str) -> String {
    let mut result = String::new();
    if name.is_empty() {
        result.push_str("ERROR: name is empty. ");
    }
    if age < 0 {
        result.push_str("ERROR: age is negative. ");
    }
    if age > 150 {
        result.push_str("ERROR: age is too large. ");
    }
    if email.is_empty() {
        result.push_str("ERROR: email is empty. ");
    }
    if !email.contains('@') {
        result.push_str("ERROR: email is invalid. ");
    }
    if result.is_empty() {
        result = format!("User {} (age {}) with email {} is valid", name, age, email);
    }
    result
}

// ============================================================
// CODE SMELL: Overly complex function / high cyclomatic complexity
// ============================================================

pub fn overly_complex_calculator(op: &str, a: f64, b: f64, c: f64, mode: &str, flag: bool) -> f64 {
    let mut result = 0.0;
    if op == "add" {
        if mode == "simple" {
            if flag {
                result = a + b;
            } else {
                result = a + b + c;
            }
        } else if mode == "double" {
            if flag {
                result = (a + b) * 2.0;
            } else {
                result = (a + b + c) * 2.0;
            }
        } else if mode == "half" {
            if flag {
                result = (a + b) / 2.0;
            } else {
                result = (a + b + c) / 2.0;
            }
        } else {
            result = a + b + c;
        }
    } else if op == "sub" {
        if mode == "simple" {
            if flag {
                result = a - b;
            } else {
                result = a - b - c;
            }
        } else if mode == "double" {
            if flag {
                result = (a - b) * 2.0;
            } else {
                result = (a - b - c) * 2.0;
            }
        } else if mode == "half" {
            if flag {
                result = (a - b) / 2.0;
            } else {
                result = (a - b - c) / 2.0;
            }
        } else {
            result = a - b - c;
        }
    } else if op == "mul" {
        if mode == "simple" {
            if flag {
                result = a * b;
            } else {
                result = a * b * c;
            }
        } else if mode == "double" {
            if flag {
                result = (a * b) * 2.0;
            } else {
                result = (a * b * c) * 2.0;
            }
        } else if mode == "half" {
            if flag {
                result = (a * b) / 2.0;
            } else {
                result = (a * b * c) / 2.0;
            }
        } else {
            result = a * b * c;
        }
    } else if op == "div" {
        if mode == "simple" {
            if flag {
                result = a / b;
            } else {
                result = a / b / c;
            }
        } else if mode == "double" {
            if flag {
                result = (a / b) * 2.0;
            } else {
                result = (a / b / c) * 2.0;
            }
        } else if mode == "half" {
            if flag {
                result = (a / b) / 2.0;
            } else {
                result = (a / b / c) / 2.0;
            }
        } else {
            result = a / b / c;
        }
    } else {
        result = 0.0;
    }
    result
}

// ============================================================
// CODE SMELL: Long parameter list / data clumps
// ============================================================

pub fn create_report(
    title: &str,
    author: &str,
    date: &str,
    department: &str,
    manager: &str,
    category: &str,
    priority: &str,
    status: &str,
    description: &str,
    notes: &str,
    version: i32,
    page_count: i32,
) -> String {
    format!(
        "Report: {} by {} on {} dept={} mgr={} cat={} pri={} status={} desc={} notes={} v{} pages={}",
        title, author, date, department, manager, category, priority, status, description, notes, version, page_count
    )
}

// ============================================================
// CODE SMELL: Duplicated report creation (more copy-paste)
// ============================================================

pub fn generate_sales_report(data: &[f64]) -> String {
    let mut total = 0.0;
    let mut count = 0;
    let mut max = f64::MIN;
    let mut min = f64::MAX;
    for val in data {
        total += val;
        count += 1;
        if *val > max { max = *val; }
        if *val < min { min = *val; }
    }
    let avg = if count > 0 { total / count as f64 } else { 0.0 };
    format!("Sales Report: total={:.2}, avg={:.2}, max={:.2}, min={:.2}, count={}", total, avg, max, min, count)
}

pub fn generate_inventory_report(data: &[f64]) -> String {
    let mut total = 0.0;
    let mut count = 0;
    let mut max = f64::MIN;
    let mut min = f64::MAX;
    for val in data {
        total += val;
        count += 1;
        if *val > max { max = *val; }
        if *val < min { min = *val; }
    }
    let avg = if count > 0 { total / count as f64 } else { 0.0 };
    format!("Inventory Report: total={:.2}, avg={:.2}, max={:.2}, min={:.2}, count={}", total, avg, max, min, count)
}

pub fn generate_expense_report(data: &[f64]) -> String {
    let mut total = 0.0;
    let mut count = 0;
    let mut max = f64::MIN;
    let mut min = f64::MAX;
    for val in data {
        total += val;
        count += 1;
        if *val > max { max = *val; }
        if *val < min { min = *val; }
    }
    let avg = if count > 0 { total / count as f64 } else { 0.0 };
    format!("Expense Report: total={:.2}, avg={:.2}, max={:.2}, min={:.2}, count={}", total, avg, max, min, count)
}

pub fn generate_revenue_report(data: &[f64]) -> String {
    let mut total = 0.0;
    let mut count = 0;
    let mut max = f64::MIN;
    let mut min = f64::MAX;
    for val in data {
        total += val;
        count += 1;
        if *val > max { max = *val; }
        if *val < min { min = *val; }
    }
    let avg = if count > 0 { total / count as f64 } else { 0.0 };
    format!("Revenue Report: total={:.2}, avg={:.2}, max={:.2}, min={:.2}, count={}", total, avg, max, min, count)
}

// ============================================================
// CODE SMELL: Magic numbers / hardcoded values
// ============================================================

pub fn calculate_shipping_cost(weight: f64, distance: f64) -> f64 {
    let base = 5.99;
    if weight > 50.0 {
        base + weight * 0.75 + distance * 0.12 + 15.0
    } else if weight > 20.0 {
        base + weight * 0.50 + distance * 0.08 + 7.50
    } else if weight > 5.0 {
        base + weight * 0.25 + distance * 0.05 + 3.25
    } else {
        base + weight * 0.10 + distance * 0.02 + 1.00
    }
}

pub fn calculate_tax(amount: f64, region: &str) -> f64 {
    if region == "US-CA" {
        amount * 0.0725
    } else if region == "US-NY" {
        amount * 0.08
    } else if region == "US-TX" {
        amount * 0.0625
    } else if region == "US-WA" {
        amount * 0.065
    } else if region == "CA-ON" {
        amount * 0.13
    } else if region == "CA-BC" {
        amount * 0.12
    } else if region == "UK" {
        amount * 0.20
    } else if region == "DE" {
        amount * 0.19
    } else if region == "FR" {
        amount * 0.20
    } else if region == "JP" {
        amount * 0.10
    } else {
        amount * 0.15
    }
}

// ============================================================
// CODE SMELL: God function that does too many things
// ============================================================

pub fn process_order(
    items: &[(String, f64, i32)],
    customer_name: &str,
    customer_email: &str,
    shipping_address: &str,
    payment_method: &str,
    coupon_code: &str,
) -> String {
    let mut subtotal = 0.0;
    for (name, price, qty) in items {
        let line_total = price * (*qty as f64);
        subtotal += line_total;
        println!("Item: {} x{} = ${:.2}", name, qty, line_total);
    }

    let discount = if coupon_code == "SAVE10" {
        subtotal * 0.10
    } else if coupon_code == "SAVE20" {
        subtotal * 0.20
    } else if coupon_code == "SAVE50" {
        subtotal * 0.50
    } else if coupon_code == "FREESHIP" {
        0.0
    } else {
        0.0
    };

    let after_discount = subtotal - discount;

    let tax = after_discount * 0.08;
    let shipping = if coupon_code == "FREESHIP" {
        0.0
    } else if after_discount > 100.0 {
        0.0
    } else if after_discount > 50.0 {
        5.99
    } else {
        9.99
    };

    let total = after_discount + tax + shipping;

    let payment_valid = if payment_method == "credit_card" {
        true
    } else if payment_method == "paypal" {
        true
    } else if payment_method == "bitcoin" {
        true
    } else if payment_method == "cash" {
        false
    } else {
        false
    };

    if !payment_valid {
        return format!("ERROR: Invalid payment method: {}", payment_method);
    }

    if customer_name.is_empty() {
        return String::from("ERROR: Customer name is required");
    }
    if customer_email.is_empty() {
        return String::from("ERROR: Customer email is required");
    }
    if !customer_email.contains('@') {
        return String::from("ERROR: Invalid email address");
    }
    if shipping_address.is_empty() {
        return String::from("ERROR: Shipping address is required");
    }

    let confirmation = format!(
        "Order confirmed for {}! Subtotal: ${:.2}, Discount: -${:.2}, Tax: ${:.2}, Shipping: ${:.2}, Total: ${:.2}. Shipping to: {}. Payment: {}.",
        customer_name, subtotal, discount, tax, shipping, total, shipping_address, payment_method
    );

    println!("Sending confirmation email to {}...", customer_email);
    println!("{}", confirmation);

    confirmation
}

// ============================================================
// CODE SMELL: Needless mutability and complex state management
// ============================================================

pub fn transform_data(input: &[i32]) -> Vec<i32> {
    let mut temp1: Vec<i32> = Vec::new();
    let mut temp2: Vec<i32> = Vec::new();
    let mut temp3: Vec<i32> = Vec::new();
    let mut result: Vec<i32> = Vec::new();

    for i in 0..input.len() {
        temp1.push(input[i] * 2);
    }

    for i in 0..temp1.len() {
        if temp1[i] > 10 {
            temp2.push(temp1[i]);
        }
    }

    for i in 0..temp2.len() {
        temp3.push(temp2[i] + 1);
    }

    for i in 0..temp3.len() {
        if temp3[i] % 2 == 0 {
            result.push(temp3[i]);
        }
    }

    result
}

// ============================================================
// CODE SMELL: Boolean parameter / flag arguments
// ============================================================

pub fn format_name(first: &str, last: &str, uppercase: bool, reverse: bool, include_middle: bool, middle: &str) -> String {
    let mut name = if reverse {
        if include_middle {
            format!("{}, {} {}", last, first, middle)
        } else {
            format!("{}, {}", last, first)
        }
    } else {
        if include_middle {
            format!("{} {} {}", first, middle, last)
        } else {
            format!("{} {}", first, last)
        }
    };

    if uppercase {
        name = name.to_uppercase();
    }

    name
}
