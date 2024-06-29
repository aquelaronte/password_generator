use crate::utils::{self, copy_to_clipboard};

use super::gen::generate;

pub fn main() {
    loop {
        let pwd_length = utils::capture_value(
            "How many characters long do you desire on your password? (default: 8)",
            Some("8"),
            |value| match value.trim().parse::<u32>() {
                Ok(res) => res,
                Err(_) => 8,
            },
        );

        let include_mayus = utils::capture_option(
            "Include mayus characters? y or n (default: y)",
            utils::BoolLiteral::Yes,
        );

        let include_sp = utils::capture_option(
            "Include special characters? y or n (default: y)",
            utils::BoolLiteral::Yes,
        );

        let include_num = utils::capture_option(
            "Include numbers? y or n (default: y)",
            utils::BoolLiteral::Yes,
        );

        let pwd = generate(pwd_length, include_mayus, include_sp, include_num);

        println!("Your generated password is:\n{}", pwd);

        let include_clipboard = utils::capture_option(
            "Copy to clipboard? y or n (default: y)",
            utils::BoolLiteral::Yes,
        );

        if include_clipboard {
            let successful = copy_to_clipboard(pwd);
            if successful {
                println!("Password copied successfully to clipboard");
            } else {
                println!("Error copying password to clipboard");
            }
        }

        let reload = utils::capture_option(
            "\nDo you wish to create another password? y or n (default: n)",
            utils::BoolLiteral::No,
        );

        if reload {
            continue;
        } else {
            println!("Have a nice day");
            break;
        }
    }
}
