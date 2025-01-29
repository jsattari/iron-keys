use clap::Parser;
use std::process::Command;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// create  passwords
struct Args {
    #[arg(short('l'), long, default_value = "8", required(false))]
    // Desired length of password
    length: String,

    #[arg(short('r'), long, required(false))]
    // Disable repeating characters
    repeats: bool,

    #[arg(short('a'), long, required(false), default_value = "")]
    // Characters to be excluded
    avoid: String,
}

fn main() {
    // parse input
    let args = Args::parse();

    println!("{:?}", args);

    // get desired len of password
    let arr_len: usize = args.length.parse().unwrap();

    // create placeholder for output
    let mut password: Vec<u32> = vec![0; arr_len];

    // create list of values representing usable chars
    let mut _ascii: Vec<u32> = (33..127).map(|n| n as u32).collect();

    // if avoid, create vec of avoidance chars and remove from ascii_chars_vec
    let avoids: Vec<u32> = args.avoid.chars().map(|c| c as u32).collect();

    for i in avoids.iter() {
        if let Some(position) = _ascii.iter().position(|x| x == i) {
            _ascii.remove(position);
        }
    }

    println!("{:?}", _ascii);

    // append n chars to vector based on length flag
    for idx in 0..arr_len {
        // get random value
        let rando: u32 = choose_rand(0, _ascii.len() as i32) as u32;

        // check if repeats
        if args.repeats {
            if let Some(position) = _ascii.iter().position(|&x| x == rando) {
                _ascii.remove(position);
            }
        }

        // append to output vector
        password[idx] = _ascii[rando as usize];

        println!("{:?}", password);
    }

    // turn into string and return password
    let password_str: String = password.iter().filter_map(|&n| char::from_u32(n)).collect();

    println!("{}", password_str)
}

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

fn choose_rand(min: i32, max: i32) -> i32 {
    // find range
    let range = max - min;

    // get enough bytes for an i32
    let rand_bytes = generate_rand_byte(1);

    // convert rand bytes into i32 val
    let rand_val = rand_bytes[0] as i32;

    // limit val to range (0, max) using modulus
    let result = rand_val % range;

    min + result
}
