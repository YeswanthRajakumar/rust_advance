fn find_largest_i32(v: &[i32]) -> &i32 {
    let mut largest_number = &v[0];
    for number in v {
        if number > largest_number {
            largest_number = number
        }
    }
   largest_number
}

fn find_largest_char(v: &[char]) -> &char {
    let mut largest_char = &v[0];
    for char in v {
        if char > largest_char {
            largest_char = char
        }
    }
    largest_char
}

fn find_largest<T>(list:&[T]) -> &T{
    let mut largest_value = &list[0];
    for value in list {
        if value > largest_value {
            largest_value = value
        }
    }
    largest_value
}


// with same generic type
#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T
}

// Generic impl block
impl<T> Point<T> {
    pub(crate) fn get_x(&self) -> &T {
        &self.x
    }
}

// f32 impl block which only works with f32 type
impl Point<f32> {
    fn distance_from_origin(&self)-> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// with different generic type
struct  Points<T,U>{
    x:T,
    y:U
}


fn main() {
    let  v1 = vec![32, 1, 4, 3, -1, 8];
    println!("{}", find_largest_i32(&v1));
    let  v2 = vec!['a','z','y','m'];
    println!("{}", find_largest_char(&v2));

    let v3 = vec!['a','z','y','m'];
    println!("{}",find_largest(&v3));

    let integer = Point{ x:1,y:5};
    let float = Point{ x:2.3,y:4.6};
    println!("{:?},{:?}",integer,float);
    println!("{}",integer.get_x());

    let _integer_float = Points{x:2,y:3.6};

}


