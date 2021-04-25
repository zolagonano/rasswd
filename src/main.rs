use getrandom::getrandom;
use std::env;

const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const PUNCTUATION: [char; 32] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

const HELP_MESSAGE: &'static str = r#"rasswd 0.1.0
Zola Gonano <zolagonano@protonmail.com>
A fast and simple password generator

USAGE: 
    rasswd [LETTERS] [PASSWORD_LENGTH]

FLAGS:
    -h, --help   Prints help information

LETTERS:
    l   Lowercase letters
    L   Uppercase letters
    p   punctuations
    d   digits

EXAMPLES:
    rasswd          # Generates a password with 45 characters that includes all letters
    rasswd lLd      # Generates a password with 45 characters that includes lowercase letters, uppercase letters, and digits.
    rasswd Lpdl 20  # Generates a password with 20 characters that includes all letters.
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] != "--help" && args[1] != "-h" && args[1] != "help" && args[1] != "h" {
            let is_lower = args[1].contains("l");
            let is_upper = args[1].contains("L");
            let is_punc = args[1].contains("p");
            let is_digit = args[1].contains("d");

            let password_length: u8 = match args.len() > 2 {
                true => args[2]
                    .parse()
                    .expect("could not generate password with this length"),
                false => 45,
            };

            pass_gen(is_lower, is_upper, is_punc, is_digit, password_length);
        } else {
            println!("{}", HELP_MESSAGE);
        }
    } else {
        pass_gen(true, true, true, true, 45);
    }
}

fn pass_gen(is_lower: bool, is_upper: bool, is_punc: bool, is_digit: bool, password_length: u8) {
    if is_lower && !is_upper && !is_punc && !is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();
            password.push(ASCII_LOWER[random_index % 26]);
        }

        println!("{}", password);
    } else if !is_lower && is_upper && !is_punc && !is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();
            password.push(ASCII_UPPER[random_index % 26]);
        }

        println!("{}", password);
    } else if !is_lower && !is_upper && is_punc && !is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();
            password.push(PUNCTUATION[random_index % 32]);
        }
        println!("{}", password);
    } else if !is_lower && !is_upper && !is_punc && is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();
            password.push(DIGITS[random_index % 10]);
        }
        println!("{}", password);
    } else if is_lower && is_upper && !is_punc && !is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 2 {
                0 => ASCII_LOWER[random_index % 26],
                1 => ASCII_UPPER[random_index % 26],
                _ => '\0',
            });
        }

        println!("{}", password);
    } else if is_lower && !is_upper && is_punc && !is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 2 {
                0 => ASCII_LOWER[random_index % 26],
                1 => PUNCTUATION[random_index % 32],
                _ => '\0',
            })
        }

        println!("{}", password);
    } else if is_lower && !is_upper && !is_punc && is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 2 {
                0 => ASCII_LOWER[random_index % 26],
                1 => DIGITS[random_index % 10],
                _ => '\0',
            })
        }
        println!("{}", password);
    } else if !is_lower && is_upper && is_punc && !is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 2 {
                0 => ASCII_UPPER[random_index % 26],
                1 => PUNCTUATION[random_index % 32],
                _ => '\0',
            })
        }

        println!("{}", password);
    } else if !is_lower && is_upper && !is_punc && is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 2 {
                0 => ASCII_UPPER[random_index % 26],
                1 => DIGITS[random_index % 10],
                _ => '\0',
            })
        }
        println!("{}", password);
    } else if !is_lower && !is_upper && is_punc && is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 2 {
                0 => PUNCTUATION[random_index % 32],
                1 => DIGITS[random_index % 10],
                _ => '\0',
            })
        }
        println!("{}", password);
    } else if is_lower && is_upper && is_punc && !is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 3 {
                0 => ASCII_LOWER[random_index % 26],
                1 => ASCII_UPPER[random_index % 26],
                2 => PUNCTUATION[random_index % 32],
                _ => '\0',
            })
        }
        println!("{}", password);
    } else if is_lower && is_upper && !is_punc && is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 3 {
                0 => ASCII_LOWER[random_index % 26],
                1 => ASCII_UPPER[random_index % 26],
                2 => DIGITS[random_index % 10],
                _ => '\0',
            })
        }
        println!("{}", password);
    } else if is_lower && !is_upper && is_punc && is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 3 {
                0 => ASCII_LOWER[random_index % 26],
                1 => PUNCTUATION[random_index % 32],
                2 => DIGITS[random_index % 10],
                _ => '\0',
            })
        }
        println!("{}", password);
    } else if !is_lower && is_upper && is_punc && is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 3 {
                0 => ASCII_UPPER[random_index % 26],
                1 => PUNCTUATION[random_index % 32],
                2 => DIGITS[random_index % 10],
                _ => '\0',
            })
        }
        println!("{}", password);
    } else if is_lower && is_upper && is_punc && is_digit {
        let mut password = String::new();

        for _ in 0..password_length {
            let random_index = rand_gen();

            password.push(match random_index % 4 {
                0 => ASCII_LOWER[random_index % 26],
                1 => ASCII_UPPER[random_index % 26],
                2 => PUNCTUATION[random_index % 32],
                3 => DIGITS[random_index % 10],
                _ => '\0',
            })
        }
        println!("{}", password);
    }
}

fn rand_gen() -> usize {
    let mut buf = [0u8; 1];
    getrandom(&mut buf).expect("could not get random number");

    buf[0] as usize
}
