extern crate seximal;

use std::io::stdin;

fn main() {
    let mut s = String::new();
    loop {
        stdin().read_line(&mut s).unwrap();
        let n = s.replace(<char>::is_whitespace, "");
        println!("{}", seximal::to_seximal_words(n.trim()).unwrap_or_else(|_| "Malformed number".to_owned()));
        s.clear();
    }
}
