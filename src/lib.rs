use std::fmt::{self, Display, Write, Error};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

use crate::Digit::*;

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

    // Remove leading zeroes
    while number.len() > 1 && number.first() == Some(&(Zero, Zero)) {
        number.remove(0);
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
            write!(buf, "dozen {} ", d2)?;
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

pub fn to_seximal_words(mut s: &str) -> Result<String, Error> {
    let prefix = if s.starts_with("-") {
        s = &s[1..];
        "negative "
    } else {
        if s.starts_with("+") {
            s = &s[1..];
        }
        ""
    };

    let number = number_from_str(s).ok_or(Error)?;

    let mut number_string = String::with_capacity(4*s.len() + prefix.len());

    number_string.push_str(prefix);

    let digit_pair_amount = number.len();

    if digit_pair_amount == 0 {
        return Err(Error)
    }

    let last_par_i = digit_pair_amount - 1;

    if digit_pair_amount == 1 {
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
                // Don't write "one" in front of nif, if it's the first digit
                if !(last_par_i == n && (d1, d2) == (Zero, One)) {
                    convert_pair(&mut number_string, d1, d2)?;
                }

                number_string.push_str("nif ");
            }
            // even numbers above 2 (-exian)
            n => {
                if (d1, d2) != (Zero, Zero) {
                    convert_pair(&mut number_string, d1, d2)?;
                } else if !do_exian {
                    continue
                }
                let mut prefix = SeximalDigits::new(n / 2);
                loop {
                    let d;
                    if let Some(digit) = prefix.next() {
                        d = digit;
                    } else {
                        break
                    }
                    number_string.push_str(match d {
                        Zero => "nil",
                        One => match prefix.peek() {
                            None | Some(One) | Some(Three) | Some(Four) | Some(Zero) => "un",
                            Some(Two) | Some(Five) => "um",
                        },
                        Two => "bi",
                        Three => "tri",
                        Four => match prefix.peek() {
                            Some(Two) | Some(Three) | Some(Four) | Some(Five) => "quada",
                            None | Some(Zero) | Some(One)  => "quad",
                        },
                        Five => match prefix.peek() {
                            Some(Two) | Some(Three) | Some(Four) | Some(Five) => "penta",
                            None | Some(Zero) | Some(One) => "pent",
                        },
                    })
                }
                number_string.push_str("exian ");
            }
        }
    }
    Ok(number_string)
}

struct SeximalDigits {
    base: usize,
    n: usize,
}

impl SeximalDigits {
    fn new(n: usize) -> Self {
        // let base = 6usize.pow((n as f32).log(6.) as u32);
        let mut base = 1;

        while base * 6 <= n {
            base *= 6;
        }

        SeximalDigits {
            base,
            n
        }
    }
    fn peek(&self) -> Option<Digit> {
        self.n.checked_div(self.base).map(Digit::from_usize)
    }
}

impl Iterator for SeximalDigits {
    type Item = Digit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.base >= 1 {
            let digit = self.n / self.base;

            self.n %= self.base;
            self.base /= 6;

            Some(Digit::from_usize(digit))
        } else {
            None
        }
    }
}
