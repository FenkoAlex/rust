fn main() {
    let s = "hello world";

    let res = first_word(s);
    let test2 = &s[0..2];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}