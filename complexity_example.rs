use std::fs::File;
use std::io::{self, Read};
use std::time::SystemTime;
use std::collections::HashMap;

fn do_stuff(x: i32, y: Option<String>) -> Result<HashMap<String, i32>, String> {
    let mut result = HashMap::new();

    let now = SystemTime::now();
    let elapsed = now.elapsed().unwrap().as_secs();

    if x > 0 {
        if let Some(v) = y {
            if v.len() > 3 {
                for i in 0..x {
                    if i % 2 == 0 {
                        let mut temp = vec![0u8; 128];
                        let mut file = match File::open("config.txt") {
                            Ok(f) => f,
                            Err(_) => return Err("No config".to_string()),
                        };
                        match file.read(&mut temp) {
                            Ok(_) => {
                                let s = match String::from_utf8(temp.clone()) {
                                    Ok(s) => s,
                                    Err(_) => return Err("UTF8 Error".to_string()),
                                };
                                let val = unsafe {
                                    let p = temp.as_ptr();
                                    *p.offset(5) as i32
                                };
                                result.insert(format!("{}-{}", v, i), val + x as i32 + elapsed as i32);
                            },
                            Err(_) => return Err("Read fail".to_string()),
                        }
                    }
                }
            } else {
                return Err("String too short".to_string());
            }
        } else {
            for j in 0..x {
                result.insert(j.to_string(), j * 42);
            }
        }
    } else if x == 0 {
        let thing = do_stuff(1, Some("abc".to_string()))?;
        for (k, v) in thing {
            result.insert(k, v);
        }
    } else {
        return Err("Negative x not supported".to_string());
    }

    Ok(result)
}
