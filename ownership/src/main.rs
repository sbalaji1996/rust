fn main() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);

    s1.push_str(", world.");

    println!("s1: {}, s2: {}", s1, s2);
}
