fn main() {
    another_function(5, 8);
    second_function();
    
    let x = five();
    println!("The value of x is: {}", x);

    let z = plus_one(x);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);
}

fn second_function() {
    let x: i32 = 5;

    let y = {
        x + 1
    };
    println!("The value of y as calculated by expression is: {}", y);
}

//expressions (no semicolon) return a value, statements (semicolon) do not
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
