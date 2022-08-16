fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    //String Slice
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let slice1 = &s[..3];

    let len = s.len();
    let slice2 = &s[3..len];
    let slice3 = &s[3..];
    let slice4 = &s[0..len]; //[..]과 동일
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
