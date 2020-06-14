#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn build_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect2 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} squared pixels",
        rect0.area(),
    );
    println!("width:{} height: {}", rect0.width, rect0.height);
    println!("rect1 is: {:#?}", rect0);

    println!("Can rect0 hold rect1? {}", rect0.can_hold(&rect1));
    println!("can rect0 hold rect2? {}", rect0.can_hold(&rect2));

    let sqr0 = Rectangle::build_square(20);
    println!("Can rect0 hold sqr0? {}", rect0.can_hold(&sqr0));

    let sqr1 = Rectangle::build_square(70);
    println!("Can rect0 hold sqr1? {}", rect0.can_hold(&sqr1));
}
