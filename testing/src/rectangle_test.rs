pub struct Rectangle{
    length:i32,
    breadth:i32,
}

impl Rectangle {
    pub fn new(width:i32,height:i32) -> Rectangle{
        Rectangle{
            length : width,
            breadth: height,
        }
    }

    pub fn can_hold(&self,rect2:Rectangle) -> bool{

        self.length > rect2.length && self.breadth > rect2.breadth

    }

}