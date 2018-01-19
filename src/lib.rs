use std::fmt::{self, Display, Write, Error};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Digit {
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
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            _ => return None
        })
    }
    fn from_usize(n: usize) -> Self {
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

fn number_from_str(s: &str) -> Option<Vec<(Digit, Digit)>> {
    let even = s.len() & 1 == 0;
    let mut iter = s.chars().map(|c| Digit::from_char(c));
    let mut number = Vec::with_capacity(s.len() / 2 + if even{0}else{1});

    if !even {
        number.push((Zero, iter.next().unwrap()?));
    }

    while let (Some(d1), Some(d2)) = (iter.next(), iter.next()) {
        number.push((d1?, d2?))
    }

    Some(number)
}

fn convert_pair(buf: &mut String, d1: Digit, d2: Digit) -> fmt::Result {
    match d1 {
        Zero => write!(buf, "{} ", d2)?,
        One => match d2 {
            Zero => buf.push_str("six "),
            One => buf.push_str("seven "),
            Two => buf.push_str("eight "),
            Three => buf.push_str("nine "),
            Four => buf.push_str("ten "),
            Five => buf.push_str("eleven "),
        },
        Two => if d2 == Zero {
            buf.push_str("twelve ")
        } else {
            buf.push_str("dozen ");
            if d2 != Zero {
                write!(buf, "{} ", d2)?;
            }
        }
        Three => {
            buf.push_str("thirsy ");
            if d2 != Zero {
                write!(buf, "{} ", d2)?;
            }
        }
        Four => {
            buf.push_str("foursy ");
            if d2 != Zero {
                write!(buf, "{} ", d2)?;
            }
        }
        Five => {
            buf.push_str("fifsy ");
            if d2 != Zero {
                write!(buf, "{} ", d2)?;
            }
        }
    }
    Ok(())
}

pub fn to_seximal_words(s: &str) -> Result<String, Error> {
    let number = number_from_str(s).ok_or(Error)?;

    let mut number_string = String::with_capacity(4*s.len());

    if number.len() == 1 {
        let (d1, d2) = number[0];
        convert_pair(&mut number_string, d1, d2)?;
        return Ok(number_string);
    }

    let mut do_exian = false;

    for (i, (d1, d2)) in number.into_iter().rev().enumerate().rev() {
        match i {
            0 => if (d1, d2) != (Zero, Zero) {
                convert_pair(&mut number_string, d1, d2)?;
            }
            // uneven numbers
            n if n & 1 == 1 => {
                if (d1, d2) == (Zero, Zero) {
                    do_exian = false;
                    continue
                }
                do_exian = true;
                convert_pair(&mut number_string, d1, d2)?;
                number_string.push_str("nif ");
            }
            // even numbers above 2 (-exian)
            n => {
                if (d1, d2) != (Zero, Zero) {
                    convert_pair(&mut number_string, d1, d2)?;
                } else if !do_exian {
                    continue
                }
                let prefix = simple_num(n / 2);
                for (i, &d) in prefix.iter().enumerate() {
                    match d {
                        Zero => number_string.push_str("nil"),
                        One => number_string.push_str(match prefix.get(i+1) {
                            None | Some(&One) | Some(&Three) | Some(&Four) | Some(&Zero) => "un",
                            Some(&Two) | Some(&Five) => "um",
                        }),
                        Two => number_string.push_str("bi"),
                        Three => number_string.push_str("tri"),
                        Four => number_string.push_str(match prefix.get(i+1) {
                            Some(&Two) | Some(&Three) | Some(&Four) | Some(&Five) => "quada",
                            None | Some(&Zero) | Some(&One)  => "quad",
                        }),
                        Five => number_string.push_str(match prefix.get(i+1) {
                            Some(&Two) | Some(&Three) | Some(&Four) | Some(&Five) => "penta",
                            None | Some(&Zero) | Some(&One) => "pent",
                        }),
                    }
                }
                number_string.push_str("exian ");
            }
        }
    }
    Ok(number_string)
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
