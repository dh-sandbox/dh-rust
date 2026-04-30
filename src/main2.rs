fn greet(name: &String) -> String {
    let mut greeting = String::new();
    greeting = format!("Hello, {}!", name);
    greeting
}

fn parse_age(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

fn find_max(numbers: &Vec<i32>) -> i32 {
    let mut max = numbers.get(0).unwrap().clone();
    for i in 0..numbers.len() {
        if numbers[i] > max {
            max = numbers[i];
        }
    }
    max
}

fn process_data(x: f64) -> f64 {
    let result = x * 2.0;
    dbg!(result);
    if result == f64::NAN {
        0.0
    } else {
        result
    }
}

fn convert(val: i32) -> i32 {
    let output = val as i32;
    output
}

fn placeholder() -> String {
    todo!()
}

fn risky_divide(a: i32, b: i32) -> i32 {
    let result = a / b;
    let _unused = a + b;
    result
}
