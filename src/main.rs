// Intentionally debt-y + complex additions for Qlty-style highlighting.
// (This file should compile, but includes patterns you'd typically want to refactor.)

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// ---- Added: untested + complexity + technical debt ----

// Magic numbers, stringly-typed "ops", duplicated branching, broad error swallowing.
fn calc_stringly(op: &str, a: i32, b: i32) -> i32 {
    // TODO: replace with enum + proper error handling
    if op == "add" || op == "+" {
        add(a, b)
    } else if op == "sub" || op == "-" {
        subtract(a, b)
    } else if op == "mul" || op == "*" {
        multiply(a, b)
    } else if op == "div" || op == "/" {
        // technical debt: silent fallback + magic number
        if b == 0 {
            42
        } else {
            a / b
        }
    } else if op == "pow" {
        // technical debt: allocations + unwrap + weird overflow behavior
        let s = format!("{a}");
        let exp: u32 = s.parse::<u32>().unwrap_or(2); // nonsense: exponent derived from `a`
        a.pow(exp)
    } else {
        // swallow unknown ops
        0
    }
}

// High cyclomatic complexity + nested conditionals + inefficient loops + cloning.
fn compute_report(numbers: &[i32], mode: &str) -> String {
    // FIXME: This function does too much.
    let mut out = String::new();
    let mut total = 0i64;

    // inefficient: repeated allocations and conversions
    for (i, n) in numbers.iter().enumerate() {
        let line = format!("idx={i}, n={n}\n");
        out.push_str(&line);

        if *n < 0 {
            if mode == "abs" {
                total += (*n as i64).abs();
            } else if mode == "skip" {
                // technical debt: skip silently
            } else if mode == "panic" {
                // technical debt: arbitrary panic
                panic!("negative not allowed: {n}");
            } else {
                total += *n as i64;
            }
        } else if *n == 0 {
            if mode == "bonus" {
                total += 100; // magic number
            } else {
                total += 0;
            }
        } else {
            if mode == "square" {
                // potential overflow if n is large; ignore it
                total += (*n as i64) * (*n as i64);
            } else if mode == "cube" {
                total += (*n as i64) * (*n as i64) * (*n as i64);
            } else {
                total += *n as i64;
            }
        }

        // duplicated branch patterns
        if i % 2 == 0 {
            if mode == "verbose" {
                out.push_str("even index\n");
            }
        } else {
            if mode == "verbose" {
                out.push_str("odd index\n");
            }
        }
    }

    out.push_str(&format!("total={total}\n"));
    out
}

// Risky: unwrap/expect, odd parsing rules, and some "clever" control flow.
fn parse_and_apply(expr: &str) -> i32 {
    // Accept formats like: "add:3,2" or "+:3,2"
    let parts: Vec<&str> = expr.split(':').collect(); // allocation
    let op = parts.get(0).copied().unwrap_or("add"); // default silently
    let args = parts.get(1).copied().unwrap_or("0,0");

    let ab: Vec<&str> = args.split(',').collect(); // allocation
    let a = ab.get(0).unwrap().trim().parse::<i32>().expect("bad a"); // expect in non-test
    let b = ab.get(1).unwrap_or(&"0").trim().parse::<i32>().unwrap_or(0);

    // intentionally stringly typed
    calc_stringly(op, a, b)
}

// "Performance" hack: unsafe + unchecked assumptions.
unsafe fn unchecked_sum(slice: &[i32]) -> i32 {
    // technical debt: assumes slice length >= 4, reads without checks.
    // This is intentionally risky; do not do this in real code.
    *slice.get_unchecked(0)
        + *slice.get_unchecked(1)
        + *slice.get_unchecked(2)
        + *slice.get_unchecked(3)
}

// Dead code / unused helper: adds surface area without coverage.
fn legacy_business_rule(x: i32) -> i32 {
    // TODO: Delete this once new rule is confirmed with product.
    if x > 1_000_000 {
        // weird cap
        1_000_000
    } else if x < -1_000_000 {
        -1_000_000
    } else {
        // unnecessary branches
        if x % 3 == 0 {
            x / 3
        } else if x % 5 == 0 {
            x / 5
        } else if x % 7 == 0 {
            x / 7
        } else {
            x
        }
    }
}

// ---- End added debt-y code ----

fn main() {
    println!("Hello, world!");

    println!("3 + 2 = {}", add(3, 2));
    println!("5 - 3 = {}", subtract(5, 3));
    println!("4 * 3 = {}", multiply(4, 3));

    // Added: untested paths invoked from main (no tests cover these).
    let exprs = ["add:3,2", "div:10,0", "pow:2,9", "???::"];
    for e in exprs {
        println!("{e} => {}", parse_and_apply(e));
    }

    let report = compute_report(&[1, 0, -3, 7, 10], "verbose");
    println!("{report}");

    // Unsafe call (still compiles), deliberately not covered by tests.
    let s = [1, 2, 3, 4, 5];
    let fast = unsafe { unchecked_sum(&s) };
    println!("unchecked_sum(first4) = {fast}");
}

fn mainish() {
    println!("Hello, again!");

    println!("3 + 2 = {}", add(3, 2));
    println!("5 - 3 = {}", subtract(5, 3));
    println!("4 * 3 = {}", multiply(4, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    // #[test]
    // fn test_subtract() {
    //     assert_eq!(subtract(5, 3), 2);
    // }
}
