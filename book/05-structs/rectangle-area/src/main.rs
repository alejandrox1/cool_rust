#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn from(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    let rect1 = Rectangle::from(30, 50);
    let rect2 = Rectangle::from(10, 40);
    let rect3 = Rectangle::from(60, 45);

    println!("Rectangle: {:?}", rect1);
    println!("Rectangle: {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
