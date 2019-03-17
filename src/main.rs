mod pyt;

use std::io;

fn main() {

    println!("Please input a number: ");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Could not read file");

    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("I crashed"),
    };

    pyt::py_engine(n);
}
