use std::cmp::Ordering;
use std::io;
use rand;
use rand::Rng;

/*
1. generate a secret number
2. get input number from user
3. guess the secret number
 */

fn generate_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..10)
}

fn get_user_guess_number() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess;
}

fn parse_input(number: String) -> i32 {
    match number.trim().parse() {
        Ok(number) => number,
        Err(_) => -1
    }
}

fn main() {
    let secret = generate_secret_number();
    loop {
        println!("Enter a number between 1 and 10 : ");
        let user_guess = get_user_guess_number();
        let user_guess = parse_input(user_guess);

        if user_guess == -1 {
            println!("Number is not valid....\n");
            continue;
        }

            match secret.cmp(&user_guess) {
                Ordering::Less => println!("less!!"),
                Ordering::Greater => println!("gret !!"),
                Ordering::Equal => {
                    println!("you Won !!");
                    break;
                }
            };
        }
}




