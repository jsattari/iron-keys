use std::collections::HashMap;
use std::process::Command;

fn generate_rand_byte(count: usize) -> Vec<u8> {
    // use randomness source from sys
    let output = Command::new("dd")
        .arg("if=/dev/urandom")
        .arg("bs=1")
        .arg(format!("count={}", count))
        .output()
        .expect("failed to read random bytes");

    output.stdout
}

fn choose_rand(available_keys: Vec<&u32>) -> u32 {
    // get enough bytes for an i32
    let rand_bytes = generate_rand_byte(1);

    // generate a randome idx
    let rand_idx = (rand_bytes[0] as usize) % available_keys.len();

    // find key that corresponds to random value, deref and return it
    *available_keys[rand_idx]
}

/// generate ascii hashmap
pub fn generate_ascii_chars(avoid: &str) -> HashMap<u32, char> {
    let mut ascii_chars: HashMap<u32, char> = (33..127).map(|n| (n, (n as u8 as char))).collect();

    if !avoid.is_empty() {
        // turn avoid string into vector of chars
        let avoid_chars: Vec<u32> = avoid.chars().map(|c| c as u32).collect();

        // remove ascii chars
        for &char_code in avoid_chars.iter() {
            ascii_chars.remove(&char_code);
        }
    }

    ascii_chars
}

/// generates password based on given flags
pub fn generate_password(
    length: usize,
    ascii_chars: &mut HashMap<u32, char>,
    repeat_flag: bool,
) -> String {
    let mut password = String::new();

    for _ in 0..length {
        let keys: Vec<&u32> = ascii_chars.keys().collect();
        let random_key = choose_rand(keys);

        if repeat_flag {
            // Allow repetition, remove the chosen character from the list
            if let Some(character) = ascii_chars.remove(&random_key) {
                password.push(character);
            }
        } else {
            // Prevent repetition, just append character to the password
            if let Some(&character) = ascii_chars.get(&random_key) {
                password.push(character);
            }
        }
    }

    password
}
