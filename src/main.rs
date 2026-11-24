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

fn generate(charset: &Vec<char>, length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();

    for _ in 0..length {
        let index = rng.gen_range(0..charset.len());
        password.push(charset[index]);
    }

    password
}
fn main() {
    let charset = build_charset(true, true, true);
    println!("Charset has {} characters", charset.len());

    for _ in 0..3 {
        let password = generate(&charset, 16);
        println!("{}", password);
    }

    let simple = build_charset(false, true, false);
    println!("Only lowercase + digits: {}", generate(&simple, 12));
}
