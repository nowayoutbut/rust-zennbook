use std::borrow::Cow;

fn capitalize(name: &str) -> Cow<str> {
    match name.chars().nth(0) {
        Some(first_char) if first_char.is_uppercase() => Cow::Borrowed(name),
        Some(first_char) => {
            let new_string = first_char
                .to_uppercase()
                .chain(name.chars().skip(1))
                .collect();

            Cow::Owned(new_string)
        }
        None => Cow::Borrowed(name),
    }
}
fn main() {
    match capitalize("hello") {
        Cow::Borrowed(s) => {
            println!("borrowed: {}", s)
        }
        Cow::Owned(s) => {
            println!("owned: {}", s)
        }
    }

    match capitalize("World") {
        Cow::Borrowed(s) => {
            println!("borrowed: {}", s)
        }
        Cow::Owned(s) => {
            println!("owned: {}", s)
        }
    }
}
