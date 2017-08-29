fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = add_option(x, y);

    println!("{}", sum.expect("error unwrapping"));
}

fn add_option(x: i8, y: Option<i8>) -> Option<i8> {
    match y {
        None => None,
        Some(i) => Some(i + x),
    }
}
