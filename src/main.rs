use clap::Parser;
mod utils;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Create passwords
struct Args {
    #[arg(
        short('l'),
        long,
        default_value = "8",
        required(false),
        help = "Desired length of password"
    )]
    // Desired length of password
    length: String,

    #[arg(short('r'), long, required(false), help = "Prevents repeat characters")]
    // Disable repeating characters
    repeats: bool,

    #[arg(
        short('a'),
        long,
        required(false),
        default_value = "",
        help = "Characters that should be excluded. Should be entered as a string like 'abc123'"
    )]
    // Characters to be excluded
    avoid: String,
}

fn main() {
    // parse input arguments
    let args = Args::parse();

    // get desired password length, handle potential parsing errors
    let password_length: usize = args.length.parse().unwrap_or_else(|_| {
        eprintln!("Invalid length value: {}", args.length);
        std::process::exit(1);
    });

    // Create list of usable ASCII characters
    let mut ascii_chars = utils::generate_ascii_chars(&args.avoid);

    // create output
    let password = utils::generate_password(password_length, &mut ascii_chars, args.repeats);

    // output
    println!("{}", password);
}
