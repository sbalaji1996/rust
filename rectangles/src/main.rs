#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle{length: size, width: size}
    }
}

fn main() {
    let rect1 = Rectangle{length: 50, width: 30};

    println!("rect1 is {:#?}", rect1); 

    println!("The area of the rectangle with width {} and length {} is {}.", 
             rect1.width,
             rect1.length,
             rect1.area());

    let rect2 = Rectangle{length: 25, width: 10};
    let rect3 = Rectangle{length: 100, width: 50};

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(45);

    println!("sq is {:#?}", sq);
}
