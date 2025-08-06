use std::collections::HashMap;

static mut USERS: Option<HashMap<u32, String>> = None;

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

fn do_lots_of_stuff_again() {
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


