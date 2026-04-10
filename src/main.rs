fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("Hello, world!");

    println!("3 + 2 = {}", add(3, 2));
    println!("5 - 3 = {}", subtract(5, 3));
    println!("4 * 3 = {}", multiply(4, 3));
    println!("10 / 3 = {:?}", divide(10, 3));
    println!("2 ^ 8 = {}", power(2, 8));
    println!("7.0 / 2.0 = {}", safe_divide(7.0, 2.0));

    let names = vec!["Alice".to_string(), "Bob".to_string()];
    println!("First: {:?}", get_first(&names));

    println!("10 % 3 = {}", modulo(10, 3));
    println!("double(5) = {}", double(5));
    println!("triple(5) = {}", triple(5));
    println!("square(5) = {}", square(5));
    println!("cube(5) = {}", cube(5));
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

fn power(base: i32, exp: u32) -> i32 {
    let mut result = 1;
    for _i in 0..exp {
        result *= base;
    }
    result
}

fn safe_divide(a: f64, b: f64) -> f64 {
    let result = a / b;
    if result.is_nan() {
        0.0
    } else {
        result
    }
}

fn get_first(items: &[String]) -> Option<&String> {
    items.first()
}

fn modulo(a: i32, b: i32) -> i32 {
    a % b
}

fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn square(x: i32) -> i32 {
    x * x
}

fn cube(x: i32) -> i32 {
    x * x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 3), 12);
    }
}
