extern crate seximal;

use std::io::stdin;

fn main() {
    let mut s = String::new();
    loop {
        stdin().read_line(&mut s).unwrap();
        println!("{}", seximal::to_seximal_words(s.trim()).unwrap());
        s.clear();
    }
}
