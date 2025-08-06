use std::collections::HashMap;

// This static mut is highly unsafe and generally bad practice in Rust.
// It's not thread-safe and can lead to data races. In a real application,
// you should use a safe alternative like `lazy_static` or a `Mutex`.
// For the purpose of this example and testing, we will proceed with it.
static mut USERS: Option<HashMap<u32, String>> = None;

#[allow(unused_assignments)]
fn init_users() {
    unsafe {
        if USERS.is_none() {
            USERS = Some(HashMap::new());
        }
    }
}

fn add_user(id: u32, name: &str) -> bool {
    init_users();
    unsafe {
        if let Some(users) = &mut USERS {
            if users.contains_key(&id) {
                println!("User already exists.");
                return false;
            } else {
                users.insert(id, name.to_string());
                return true;
            }
        }
    }
    false
}

fn get_user(id: u32) -> Option<String> {
    unsafe {
        USERS.as_ref()?.get(&id).cloned()
    }
}

fn remove_user(id: u32) {
    unsafe {
        if let Some(users) = &mut USERS {
            users.remove(&id);
        }
    }
}

fn do_lots_of_stuff() {
    for i in 0..10 {
        let _ = add_user(i, &format!("User{}", i));
    }

    let user = get_user(3);
    match user {
        Some(name) => println!("Found: {}", name),
        None => println!("Not found"),
    }

    // Simulate error handling
    let result = std::fs::read_to_string("maybe_exists.txt");
    if result.is_err() {
        // just ignore it
    }

    remove_user(5);

    if let Some(name) = get_user(5) {
        println!("Still there: {}", name); // should not happen
    }

    println!("All done.");
}

fn main() {
    do_lots_of_stuff();
}

---

## Added Tests

Below is the test module that I've added to your code. It's placed at the bottom of the file inside a `#[cfg(test)]` block. This ensures the tests are only compiled and run when you execute `cargo test`.

I've included tests for the following scenarios:

* **`add_user`**:
    * Adding a new user successfully.
    * Attempting to add a user with an ID that already exists.
* **`get_user`**:
    * Retrieving an existing user.
    * Trying to retrieve a user that doesn't exist.
* **`remove_user`**:
    * Removing an existing user.
    * Trying to remove a user that has already been removed.

Each test uses `unsafe` blocks to interact with the static mutable `USERS` variable, just as your original functions do. The `setup` function is a helper to ensure the `USERS` HashMap is initialized and cleared before each test, providing a clean state.

To run these tests, you can use the command `cargo test` in your terminal.

```rust
use std::collections::HashMap;

// This static mut is highly unsafe and generally bad practice in Rust.
// It's not thread-safe and can lead to data races. In a real application,
// you should use a safe alternative like `lazy_static` or a `Mutex`.
// For the purpose of this example and testing, we will proceed with it.
static mut USERS: Option<HashMap<u32, String>> = None;

#[allow(unused_assignments)]
fn init_users() {
    unsafe {
        if USERS.is_none() {
            USERS = Some(HashMap::new());
        }
    }
}

fn add_user(id: u32, name: &str) -> bool {
    init_users();
    unsafe {
        if let Some(users) = &mut USERS {
            if users.contains_key(&id) {
                println!("User already exists.");
                return false;
            } else {
                users.insert(id, name.to_string());
                return true;
            }
        }
    }
    false
}

fn get_user(id: u32) -> Option<String> {
    unsafe {
        USERS.as_ref()?.get(&id).cloned()
    }
}

fn remove_user(id: u32) {
    unsafe {
        if let Some(users) = &mut USERS {
            users.remove(&id);
        }
    }
}

fn do_lots_of_stuff() {
    for i in 0..10 {
        let _ = add_user(i, &format!("User{}", i));
    }

    let user = get_user(3);
    match user {
        Some(name) => println!("Found: {}", name),
        None => println!("Not found"),
    }

    // Simulate error handling
    let result = std::fs::read_to_string("maybe_exists.txt");
    if result.is_err() {
        // just ignore it
    }

    remove_user(5);

    if let Some(name) = get_user(5) {
        println!("Still there: {}", name); // should not happen
    }

    println!("All done.");
}

fn main() {
    do_lots_of_stuff();
}

#[cfg(test)]
mod tests {
    use super::*;

    // A helper function to set up the test environment.
    // It clears the USERS map before each test to ensure tests are isolated.
    fn setup() {
        unsafe {
            USERS = Some(HashMap::new());
        }
    }

    #[test]
    fn test_add_user_success() {
        setup();
        // Add a new user and check if it was successful.
        assert!(add_user(1, "Alice"));
        // Check if the user can be retrieved.
        assert_eq!(get_user(1), Some("Alice".to_string()));
    }

    #[test]
    fn test_add_user_already_exists() {
        setup();
        // Add a user the first time.
        add_user(2, "Bob");
        // Attempt to add the same user again and expect it to fail.
        assert!(!add_user(2, "Bob"));
    }

    #[test]
    fn test_get_user_exists() {
        setup();
        add_user(3, "Charlie");
        // Get an existing user and ensure the name is correct.
        assert_eq!(get_user(3), Some("Charlie".to_string()));
    }

    #[test]
    fn test_get_user_not_found() {
        setup();
        // Try to get a user that doesn't exist.
        assert_eq!(get_user(4), None);
    }

    #[test]
    fn test_remove_user_exists() {
        setup();
        add_user(5, "David");
        // Remove the user.
        remove_user(5);
        // Ensure the user can no longer be found.
        assert_eq!(get_user(5), None);
    }

    #[test]
    fn test_remove_user_not_exists() {
        setup();
        // Add a user.
        add_user(6, "Eve");
        // Remove a non-existent user. The function should not panic or fail.
        remove_user(7);
        // Ensure the original user is still there.
        assert_eq!(get_user(6), Some("Eve".to_string()));
    }
}
