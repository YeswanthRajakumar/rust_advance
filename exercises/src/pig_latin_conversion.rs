fn is_vowel(s: &str) -> bool {
    let c:Vec<char> = s.to_lowercase().chars().collect();
    ['a','e','i','o','u'].contains(&c[0])
}



pub fn get_result(s: &mut str) {
    let mut result  = String::new();
    match is_vowel(s){
        true => result = pig_latin_vowel_conversion(s),
        false =>result = pig_latin_consonant_conversion(s),
    };

    println!("{}",result)
}


fn pig_latin_vowel_conversion(s: &str) -> String {
   s.to_owned() + "-hay"
}

fn pig_latin_consonant_conversion(s: &str) -> String {
    let first_char = s.to_string().remove(0);
    let mut str_to_append = "-ay".to_string();
    str_to_append.insert(1,first_char);
    s[1..s.len()].to_owned() + str_to_append.as_str()
}
