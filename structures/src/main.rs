// STRUCT DEFINITION
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn display(user: User) {
    println!("Username : {}\nEmail : {}\nactive :{}\nsign_in_count:{}\n",
             user.name, user.email, user.active, user.sign_in_count, )
}

// TUPLE STRUCT
struct Color(u64, u64, u64);

struct LatLong(f32, f32);

fn main() {
    // STRUCT INSTANTIATION
    let mut _user1 = User {
        name: String::from("Yeswanth"),
        email: String::from("yeswanthjayanthi@gmail.com"),
        sign_in_count: 0,
        active: false,
    };

    // Building STRUCT using function
    let mut user2 = build_user(String::from("Arun"),
                               String::from("yeswanth@gmail.com"));
    // println!("{}",user2.email);
    let _inactive_user2 = User {
        active: false,
        ..user2
    };
    // display(inactive_user2);

    // Tuple Struct
    let _rgb = Color(123, 23, 240);
    // println!("{}", rgb.0);
}

