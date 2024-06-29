use std::io;

use clipboard::{ClipboardContext, ClipboardProvider};

pub fn capture_value<T>(message: &str, default_value: Option<&str>, parser: fn(String) -> T) -> T {
    let ask_value = || {
        let mut option = String::new();
        println!("{}", message);

        io::stdin()
            .read_line(&mut option)
            .expect("Error reading line");

        if option.trim().is_empty() {
            match default_value {
                None => option,
                Some(value) => String::from(value),
            }
        } else {
            option
        }
    };

    parser(ask_value())
}

pub enum BoolLiteral {
    Yes,
    No,
}

impl BoolLiteral {
    fn to_str(&self) -> &str {
        match self {
            BoolLiteral::Yes => "y",
            BoolLiteral::No => "n",
        }
    }
}

pub fn capture_option(message: &str, default: BoolLiteral) -> bool {
    capture_value(message, Some(default.to_str()), |value| {
        match value.trim().to_lowercase().as_str() {
            "y" | "yes" => true,
            "n" | "no" => false,
            _ => true,
        }
    })
}

pub fn copy_to_clipboard(content: String) -> bool {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    match ctx.set_contents(content) {
        Err(_) => false,
        Ok(_) => true,
    }
}
