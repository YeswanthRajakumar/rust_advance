#[derive(Debug)]
struct RectangleT(i32, i32);

// Instance Method
impl RectangleT {
    fn area_of_rectangle(&self) -> i32 {
        self.0 * self.1
    }

    fn can_hold(&self, rect: &RectangleT) -> bool {
        self.0 >= rect.0 && self.1 >= rect.1
    }

    fn square(size:i32) -> RectangleT{
        RectangleT(size,size)
    }
}


fn main() {
    // let rect_1 = Rectangle{
    //     length : 10,
    //     breadth: 3,
    // };
    let rect_1 = RectangleT(10, 3);
    let _area = rect_1.area_of_rectangle();
    // println!("AREA of {:?}  is  {}",rect,area);

    let rect_2 = RectangleT(60, 2);
    // println!("{}",rect_1.can_hold(&rect_2));

    let sq = RectangleT::square(5);
    println!("{:?}",sq);

}

