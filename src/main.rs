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
    // init list of ascii chars
    let _ascii: Vec<char> = (33..128)
        .map(|c| std::char::from_u32(c as u32).unwrap())
        .collect();

    // output
    let mut password: Vec<char> = Vec::new();

    // parse input
    let args = Args::parse();

    let arr_len: usize = args.length.parse().unwrap();

    // append n chars to vector based on length flag
    for _ in 0..arr_len {
        // get random value
        let rando: usize = choose_rand(0, 95) as usize;

        // append to output vector
        password.push(_ascii[rando]);
    }

    // turn into string and return password
    let password_str: String = password.into_iter().collect();

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
