
fn get_first_word(s: &mut String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[0..bytes.len()]
}


fn main() {
    let mut s = String::from("Hello Wor");
    let x= get_first_word(&mut s);
    display(s);
    println!("{}",x);


}

fn display(s: String) {
    println!()
}


