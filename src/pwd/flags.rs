use clap::{command, Parser};

use crate::utils::copy_to_clipboard;

use super::gen::generate;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value_t = 8)]
    length: u32,

    #[arg(short, long, default_value_t = false, help = "include numbers")]
    numbers: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "include special characters"
    )]
    special: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "include mayus characters"
    )]
    mayus: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "copy password on clipboard"
    )]
    clipboard: bool,
}

pub fn main() {
    let parsed_args = Args::parse();

    let pwd = generate(
        parsed_args.length,
        parsed_args.mayus,
        parsed_args.special,
        parsed_args.numbers,
    );

    println!("Your generated password is:\n{}", pwd);

    if parsed_args.clipboard {
        let successful = copy_to_clipboard(pwd);
        if successful {
            println!("Password copied successfully to clipboard");
        } else {
            println!("Error copying password to clipboard");
        }
    }
}
