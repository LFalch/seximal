use std::io::stdin;
use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

use Digit::*;

impl Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Zero => "zero".fmt(f),
            One => "one".fmt(f),
            Two => "two".fmt(f),
            Three => "three".fmt(f),
            Four => "four".fmt(f),
            Five => "five".fmt(f)
        }
    }
}

impl Digit {
    pub fn from_char(c: char) -> Self {
        match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            _ => panic!("Invalid digit `{}'", c)
        }
    }
    pub fn from_usize(n: usize) -> Self {
        match n {
            0 => Zero,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            _ => panic!("Invalid digit `{}'", n)
        }
    }
    pub fn to_num(&self) -> u8 {
        match *self {
            Zero => 0,
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
        }
    }
}

fn number_from_str(s: &str) -> Vec<(Digit, Digit)> {
    let even = s.len() & 1 == 0;
    let mut iter = s.chars().map(|c| Digit::from_char(c));
    let mut number = Vec::with_capacity(s.len() / 2 + if even{0}else{1});

    if !even {
        number.push((Zero, iter.next().unwrap()));
    }

    while let (Some(d1), Some(d2)) = (iter.next(), iter.next()) {
        number.push((d1, d2))
    }

    number
}

fn main() {
    loop {
        convert_stdin()
    }
}

fn convert_pair(d1: Digit, d2: Digit) {
    match d1 {
        Zero => print!("{} ", d2),
        One => match d2 {
            Zero => print!("six "),
            One => print!("seven "),
            Two => print!("eight "),
            Three => print!("nine "),
            Four => print!("ten "),
            Five => print!("eleven "),
        },
        Two => if d2 == Zero {
            print!("twelve ")
        } else {
            print!("dozen ");
            if d2 != Zero {
                print!("{} ", d2);
            }
        }
        Three => {
            print!("thirsy ");
            if d2 != Zero {
                print!("{}", d2);
            }
        }
        Four => {
            print!("foursy ");
            if d2 != Zero {
                print!("{}", d2);
            }
        }
        Five => {
            print!("fifsy ");
            if d2 != Zero {
                print!("{}", d2);
            }
        }
    }
}

fn convert_stdin() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let number = number_from_str(s.trim());

    if number.len() == 1 {
        let (d1, d2) = number[0];
        convert_pair(d1, d2);
        return println!();
    }

    for (i, (d1, d2)) in number.into_iter().rev().enumerate().rev() {
        match i {
            0 => if (d1, d2) != (Zero, Zero) {
                convert_pair(d1, d2);
            }
            // uneven numbers
            n if n & 1 == 1 => {
                if (d1, d2) == (Zero, Zero) {
                    continue
                }
                convert_pair(d1, d2);
                print!("nif ");
            }
            // even numbers above 2 (-exian)
            n => {
                if (d1, d2) == (Zero, Zero) {
                    continue
                }
                convert_pair(d1, d2);
                let prefix = simple_num(n / 2);
                for (i, &d) in prefix.iter().enumerate() {
                    match d {
                        Zero => print!("nil"),
                        One => print!("{}", match prefix.get(i+1) {
                            None | Some(&One) | Some(&Three) | Some(&Four) | Some(&Zero) => "un",
                            Some(&Two) | Some(&Five) => "um",
                        }),
                        Two => print!("bi"),
                        Three => print!("tri"),
                        Four => print!("{}", match prefix.get(i+1) {
                            Some(&Two) | Some(&Three) | Some(&Four) | Some(&Five) => "quada",
                            None | Some(&Zero) | Some(&One)  => "quand",
                        }),
                        Five => print!("{}", match prefix.get(i+1) {
                            Some(&Two) | Some(&Three) | Some(&Four) | Some(&Five) => "penta",
                            None | Some(&Zero) | Some(&One) => "pent",
                        }),
                    }
                }
                print!("exian ");
            }
        }
    }
    println!();
}

fn simple_num(mut n: usize) -> Vec<Digit> {
    let mut num = Vec::new();
    let mut base = 6;

    loop {
        let digit = n % base;
        num.push(Digit::from_usize(digit));

        n /= 6;
        if n == 0 {
            break
        }
        base *= 6;
    }

    num.reverse();
    num
}
