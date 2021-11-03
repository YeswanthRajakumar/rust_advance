fn main() {
    // Explicitly declaring the type
    let mut v = Vec::new();

    // Letting rust infer the type
    let mut v1 = vec![1,2,3,4,5,6];

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(50);
    // println!("{:?}",v);

    {
        let scoped_v = vec![1.2,4.5,9.00];
        // freed when goes out of scope
    }

    let third_element = match v.get(2){
        Some(i) => {i}
        None => panic!("No element found..")
    };
    // println!("{:?}",&v[0..2]);
    // println!("complete");

    immutable_borrow_and_mutable_borrow_in_same_scope();

    loop_over_vecs();

    storing_different_type_in_vectors_using_enums();

}

#[derive(Debug)]
enum SpreadSheetCellType{
    Int(i32),
    String(String),
    Float(f32),
    Bool(bool),
}

fn storing_different_type_in_vectors_using_enums() {
    let mut  row = Vec::new();
    row.push(SpreadSheetCellType::Int(24));
    row.push(SpreadSheetCellType::String(String::from("Yeswanth")));
    row.push(SpreadSheetCellType::Float(6.5));
    row.push(SpreadSheetCellType::Bool(true));


    for i in row{
        println!("{:?}",i);
    }
}

fn loop_over_vecs() {
    let mut v = vec![1,2,3,4,5,6];
    // indexing immutable
    for i in &v{
        println!("{}",i);
    }

    // indexing mutable
    for i in &mut v{
        *i += 50;
        // println!("{}",i);
    }
}

fn immutable_borrow_and_mutable_borrow_in_same_scope(){
    let mut v = vec![1,2,3,4,5];
    let first = &v[0]; // immutable ref
    v.insert(0,6); // mutable ref
    // println!("{:?}",v)
}
