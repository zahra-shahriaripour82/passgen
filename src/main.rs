use rand::Rng;

fn build_charset(use_upper: bool, use_digits: bool, use_symbols: bool) -> Vec<char> {
    let mut charset = Vec::new();
    add_chars(&mut charset, "abcdefghijklmnopqrstuvwxyz");
    if use_upper {
        add_chars(&mut charset, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_digits {
        add_chars(&mut charset, "0123456789")
    };
    if use_symbols {
        add_chars(&mut charset, "!@#$%^&*-_=+?")
    };
    charset
}

fn add_chars(charset: &mut Vec<char>, chars: &str) {
    for c in chars.chars() {
        charset.push(c);
    }
}

fn generate(charset: &[char], length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();

    for _ in 0..length {
        let index = rng.gen_range(0..charset.len());
        password.push(charset[index]);
    }

    password
}

fn count_kinds(password: &str) -> (usize, usize, usize, usize) {
    let mut lower = 0;
    let mut upper = 0;
    let mut digits = 0;
    let mut symbols = 0;
    for c in password.chars() {
        if c.is_ascii_lowercase() {
            lower += 1
        }
        if c.is_ascii_uppercase() {
            upper += 1
        }
        if c.is_ascii_digit() {
            digits += 1
        } else {
            symbols += 1
        }
    }

    (lower, upper, digits, symbols)
}

fn add_to_history(history: &mut Vec<String>, password: String) {
    history.push(password);
}

fn print_history(history: &[String]) {
    println!("--- History ({} passwords) ---", history.len());

    for i in 0..history.len() {
        println!("{}. {}", i + 1, history[i])
    }
}

fn main() {
    let charset = build_charset(true, true, true);
    println!("Charset has {} characters", charset.len());
    let mut history = Vec::new();
    for _ in 0..3 {
        let password = generate(&charset, 16);
        let (lower, upper, digits, symbols) = count_kinds(&password);
        println!(
            "{}  (lower: {}, upper: {}, digits: {}, symbols: {})",
            password, lower, upper, digits, symbols
        );
        add_to_history(&mut history, password);
    }

    print_history(&history);

    let simple = build_charset(false, true, false);
    println!("Only lowercase + digits: {}", generate(&simple, 12));
}
