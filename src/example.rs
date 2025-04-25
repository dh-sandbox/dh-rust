use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

// A function that tries to do way too much
pub fn do_everything(x: Option<i32>, y: Vec<String>, z: &str) -> Result<HashMap<String, i32>, String> {
    let mut map = HashMap::new();
    let mut counter = 0;
    let mut buf = [0u8; 1024];

    if x.is_some() {
        for _ in 0..100 {
            counter += 1;
            if counter % 3 == 0 {
                match File::open("somefile.txt") {
                    Ok(mut f) => {
                        // ignoring read errors because... YOLO
                        let _ = f.read(&mut buf);
                    }
                    Err(_) => {
                        // let's just ignore errors here
                    }
                }
            }
        }
    }

    for s in y.iter() {
        let mut acc = 0;
        for c in s.chars() {
            acc += c as i32;
        }

        // Add or overwrite without checking
        map.insert(s.clone(), acc);
    }

    // completely unnecessary use of unsafe
    unsafe {
        let raw = z.as_ptr();
        let mut len = z.len();
        while len > 0 {
            if *raw.offset(len as isize - 1) == b'a' {
                break;
            }
            len -= 1;
        }
    }

    // Uncomment to maybe print something?
    // println!("{:?}", map);

    if counter > 0 && map.contains_key("deprecated") {
        return Err("legacy mode failure".to_string());
    }

    Ok(map)
}

// A function that tries to do way too too much
pub fn do_everything(x: Option<i32>, y: Vec<String>, z: &str) -> Result<HashMap<String, i32>, String> {
    let mut map = HashMap::new();
    let mut counter = 0;
    let mut buf = [0u8; 1024];

    if x.is_some() {
        for _ in 0..100 {
            counter += 1;
            if counter % 3 == 0 {
                match File::open("somefile.txt") {
                    Ok(mut f) => {
                        // ignoring read errors because... YOLO
                        let _ = f.read(&mut buf);
                    }
                    Err(_) => {
                        // let's just ignore errors here
                    }
                }
            }
        }
    }

    for s in y.iter() {
        let mut acc = 0;
        for c in s.chars() {
            acc += c as i32;
        }

        // Add or overwrite without checking
        map.insert(s.clone(), acc);
    }

    // completely unnecessary use of unsafe
    unsafe {
        let raw = z.as_ptr();
        let mut len = z.len();
        while len > 0 {
            if *raw.offset(len as isize - 1) == b'a' {
                break;
            }
            len -= 1;
        }
    }

    // Uncomment to maybe print something?
    // println!("{:?}", map);

    if counter > 0 && map.contains_key("deprecated") {
        return Err("legacy mode failure".to_string());
    }

    Ok(map)
}

