fn main() {
    let length1 = 50;
    let width1 = 30;

    println!("The area of the rectangle with width {} and length {} is {}.", 
             width1,
             length1,
             area(length1, width1));
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}
