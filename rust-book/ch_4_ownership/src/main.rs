fn main() {
    what_is_ownership();
}

fn what_is_ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let s1 = String::from("hello");
    takes_ownership(s1);
    // println!("{s1}"); // This fails because of the owernship change.

    let s1 = String::from("borrowed string");
    let s1 = takes_and_gives_back(s1);
    println!("{s1}");

    let s1 = String::from("sizable string");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");

    let mut s1 = String::from("sizable string");
    let len = calculate_length_ref(&mut s1);
    println!("(ref) The length of '{s1}' is {len}.");

    let s1 = String::from("first second third");
    let first = first_word(&s1);
    println!("The first word of '{s1}' is '{first}'");
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn takes_and_gives_back(s: String) -> String {
    println!("borrowing \"{s}\"");
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &mut String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
