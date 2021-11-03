
fn combine_strings_with_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{} - {} - {}",s1,s2,s3);
    println!("{}",s)
}

fn combine_strings_with_plus() {
    let s1 = String::from("Foo");
    let s2 = String::from("Bar");
    let s3 = s1 + &s2;
    println!("{}",s3);
    // println!("{}",s1);
    // println!("{}",s2);

}

fn iterating_over_strings() {
    let s = String::from("नमस्ते दुनिया");
    for c in s.chars(){
        // println!("{}",c);
    }
    for c in s.bytes(){
        // println!("{}",c);
    }
}

fn main() {
    //New Empty String
    let s1 = String::new();

    // Mutable String
    let s2 = String::from("Hello World");
    // println!("{}",s2);

    // converting String from string literal
    let s3 = "initial content";
    let s3 = s3.to_string();

    let mut s1 = String::from("Hello");

    // For appending string
    s1.push_str(" World");
    // println!("{}",s1);

    // For appending char
    s1.push('!');
    // println!("{}",s1);

    combine_strings_with_plus();
    combine_strings_with_format();
    iterating_over_strings();


}


