use std::collections::HashMap;

fn vector_of_tuples_to_hashmap() {
    let teams:Vec<&str> = vec!["Blue","Red","Green"];
    let score:Vec<i32> = vec![10,7,5];

    let mut scores: HashMap<&str,i32> =
        teams.into_iter().zip(score.into_iter()).collect();

    println!("{:?}",scores)
}

fn ownership_in_hashmaps() {
    let mut  map = HashMap::new();

    let key = String::from("Name");
    let value= String::from("Yeswanth");

    map.insert(key,value);
    println!("{:?}",map);
    // println!("{}",key); // Won't compile
    // println!("{}",value); // Won't compile
}

fn iterating_over_hashmap() {
    let mut  scores = HashMap::new();
    scores.insert("Red",10);
    scores.insert("Blue",15);
    scores.insert("Green",18);

    for (team,score) in &scores{
        println!("{} : {}",team,score);
    }


}

fn overwriting_a_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}




fn main() {
    let mut  scores = HashMap::new();
    scores.insert("Red",10);
    scores.insert("Blue",15);
    // println!("{:?}",scores);

    // Accessing values in a map
    let x = scores.get("Red").unwrap();

    // vector_of_tuples_to_hashmap();
    // ownership_in_hashmaps();
    // iterating_over_hashmap();

    //UPDATING A MAPP
    // overwriting_a_value();
    only_inserting_this_value_if_key_has_no_value();
}

fn only_inserting_this_value_if_key_has_no_value() {
    let mut scores = HashMap::new();
    scores.insert("Red",10);

    scores.entry("Yellow").or_insert(50);
    scores.entry("Red").or_insert(50);

    println!("{:?}", scores);
}

