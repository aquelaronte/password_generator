use rand::Rng;

const SPECIAL: [char; 32] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

pub fn generate(
    length: u32,
    include_mayus: bool,
    include_special: bool,
    include_num: bool,
) -> String {
    let mut pwd = String::new();

    let mut characters: Vec<char> = ('a'..='z').collect();

    if include_mayus {
        characters.extend(('A'..='Z').collect::<Vec<char>>());
    }

    if include_num {
        characters.extend(('0'..='9').collect::<Vec<char>>());
    }

    if include_special {
        characters.extend(SPECIAL);
    }

    let mut rng = rand::thread_rng();
    for _ in 0..=length - 1 {
        let random_index = rng.gen_range(0..characters.len());

        let random_char = characters.get(random_index);

        match random_char {
            None => pwd.push_str(""),
            Some(value) => pwd.push_str(value.to_string().as_str()),
        }
    }

    pwd
}
