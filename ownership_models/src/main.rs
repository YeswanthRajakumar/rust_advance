fn calculate_length(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

fn take_ownership_compound_type(s: String) {
    println!("Taken ownership of String {}", s);
}

fn take_ownership_scalar_type(a: i32) {
    println!("Taken ownership of i32 {}", a);
}

fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
}

fn change_word(s: &mut String) {
    s.push_str(" -> M1!");
}

fn main() {
    let mut s = String::from("Hello");
    s += " world";
    s.push_str("___o");
    // println!("{}",s);

    // copy the value from x to y because i32 is stored in stack
    let x = 5;
    let _y = x;

    //MOVE
    // Moves the value from s1 to s2 because String is stored in Heap
    let s1 = "hai";
    let _s2 = s1;
    // println!("{}",s1); // throws compilation error because s1 has moved to s2
    // println!("{}",s2);

    //CLONE
    let s1 = String::from("Apple");
    let _s2 = s1.clone();
    // println!("{} , {}",s1,s2);

    // OWNERSHIP IN FUNCTIONS
    let a = 5;
    // take_ownership_scalar_type(a);
    // println!("is {} still accessible yes",a);

    let s = String::from("OWL");
    // take_ownership_compound_type(s);
    // println!("is {} still accessible yes",s); // won't compile

    //TRANSFERRING OWNERSHIP VIA RETURN TUPLE
    let ss = String::from("Helllo");
    let (ss, sl) = calculate_length(ss);
    // println!("{}",ss);

    //BORROWING
    let sa = String::from("Helllo reference");
    let sla = calculate_length_with_ref(&sa);
    // println!("{}",sa);
    // println!("{}",sla);

    //MUTABLE BORROW
    let mut sa = String::from("Apple");
    change_word(&mut sa);
    // println!("{:?}",sa);
}
