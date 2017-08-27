use std::vec::Vec;

fn main() {
    let fib_index: usize = get_index();
    println!("The fibonacci index this program calculates is: {}", fib_index);

    let mut vec = Vec::new();
    vec.push(0);
    vec.push(1);

    if fib_index == 0 {
        println!("The fibonacci number at that index is {}", vec[0]);
    } else if fib_index == 1 {
        println!("The fibonacci number at that index is {}", vec[1]);
    } else {
        println!("{}", vec.len());
        while vec.len() != fib_index {
            let temp: usize = vec[vec.len() - 1] + vec[vec.len() - 2];
            vec.push(temp);
            println!("Adding {} to vector.", temp);
        }
        println!("The fibonacci number at {} is {}", fib_index, vec[vec.len() -1]);
    }
}

fn get_index() -> usize {
    35
}
