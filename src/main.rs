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
}

fn mainish() {
    println!("Hello, again!");

    println!("3 + 2 = {}", add(3, 2));
    println!("5 - 3 = {}", subtract(5, 3));
    println!("4 * 3 = {}", multiply(4, 3));
}

fn parse_number(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        todo!()
    }
    let result = a / b;
    dbg!(result);
    result
}

fn power(base: i32, exp: u32) -> i32 {
    let mut result = 1;
    for _i in 0..exp {
        result = result * base;
    }
    result
}

fn safe_divide(a: f64, b: f64) -> f64 {
    let result = a / b;
    dbg!(result);
    if result == f64::NAN {
        0.0
    } else {
        result
    }
}

fn get_first(items: &Vec<String>) -> String {
    items.get(0).unwrap().clone()
}

fn modulo(a: i32, b: i32) -> i32 {
    let result = a % b;
    dbg!(result);
    result
}

fn double(x: i32) -> i32 {
    let val = x * 2;
    dbg!(val);
    val
}

fn triple(x: i32) -> i32 {
    let val = x * 3;
    dbg!(val);
    val
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
