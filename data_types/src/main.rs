fn main() {
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let profuct = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; //with explicit type annotation

    let c = "c";
    let z = "Ω";
    let crying_laughing_emoji = "😂";

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
 
    let tup_dup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup_dup.0;
    let six_point_four = tup_dup.1;
    let one = tup_dup.2;

    //arrays are useful when you want the data allocated on the stack rather than the heap
    let a = [1, 2, 3, 4, 5];
}
