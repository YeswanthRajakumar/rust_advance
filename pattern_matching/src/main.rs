use crate::UsState::Alabama;

enum UsState{
    Alabama,
    Boston,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,

        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,

    }
}

fn main() {
    // println!("{}", value_in_cent(Coin::Quarter(UsState::Alabama)));
    let x = plus_one(Some(5));
    let y = plus_one(None);
    // println!("{:?}",x);
    let some_i32_value = 0u8;
    match some_i32_value { _ => {} }
}

fn plus_one(i : Option<i32>) -> Option<i32> {
    match i {
        Some(i) => Some(i + 1),
        None => None
    }
}
