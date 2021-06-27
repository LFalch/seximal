use std::ops::{Add, Sub, SubAssign};

use crate::Digit as SeximalDigit;

pub type Digits = Vec<Digit>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

use self::Digit::*;

impl Digit {
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            _ => return None
        })
    }
    pub fn from_byte_char(b: u8) -> Option<Self> {
        Some(match b {
            b'0' => Zero,
            b'1' => One,
            b'2' => Two,
            b'3' => Three,
            b'4' => Four,
            b'5' => Five,
            b'6' => Six,
            b'7' => Seven,
            b'8' => Eight,
            b'9' => Nine,
            _ => return None
        })
    }
}

impl Sub for Digit {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self - (rhs as u8)
    }
}
impl Sub<u8> for Digit {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        unsafe { std::mem::transmute(self as u8 - rhs) }
    }
}
impl SubAssign<u8> for Digit {
    fn sub_assign(&mut self, rhs: u8) {
        *self = unsafe { std::mem::transmute(*self as u8 - rhs) };
    }
}
impl Add for Digit {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        self + (rhs as u8)
    }
}
impl Add<u8> for Digit {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let res = self as u8 + rhs;

        if res > 9 {
            panic!("Result has two digits");
        }

        unsafe { std::mem::transmute(res) }
    }
}

pub fn digits_from_str<S: Into<String> + AsRef<str>>(s: S) -> Option<Digits> {
    if s.as_ref().bytes().any(|b| !(b'0'..=b'9').contains(&b)) {
        None
    } else {
        Some(unsafe {digits_from_str_unchecked(s)})
    }
}
#[inline(always)]
pub unsafe fn digits_from_str_unchecked<S: Into<String>>(s: S) -> Digits {
    let mut bs = s.into().into_bytes();

    bs.iter_mut().for_each(|b| *b -= b'0');

    std::mem::transmute(bs)
}
pub fn digits_from_num(s: u128) -> Digits {
    unsafe { digits_from_str_unchecked(format!("{}", s)) }
}

fn digit_div_rem_lt60_6(digits: (Digit, Digit)) -> (Digit, Digit) {
    match digits {
        // 0, 6
        (Zero, d2) => if d2 < Six {
            (Zero, d2)
        } else {
            (One, d2 - 6)
        }
        // 6, 12, 18
        (One, d2) => if d2 < Two {
            (One, d2 + 4)
        } else if d2 < Eight {
            (Two, d2 - 2)
        } else {
            (Three, d2 - 8)
        }
        // 18, 24
        (Two, d2) => if d2 < Four {
            (Three, d2 + 2)
        } else {
            (Four, d2 - 4)
        }
        // 30 36
        (Three, d2) => if d2 < Six {
            (Five, d2)
        } else {
            (Six, d2 - 6)
        }
        // 36 42 48
        (Four, d2) => if d2 < Two {
            (Six, d2 + 4)
        } else if d2 < Eight {
            (Seven, d2 - 2)
        } else {
            (Eight, d2 - 8)
        }
        // 48 54
        (Five, d2) => if d2 < Four {
            (Eight, d2 + 2)
        } else {
            (Nine, d2 - 4)
        }
        (Six, _) | (Seven, _) | (Eight, _) | (Nine, _) => unreachable!()
    }
}

pub fn convert_to_seximal(digits: Digits) -> Vec<SeximalDigit> {
    let mut dividend = digits;

    let mut quotient = Digits::with_capacity(dividend.len());
    let mut sex_digits = Vec::new();

    let leading_zeroes = dividend.iter().position(|&d| d!=Zero).unwrap_or_else(|| dividend.len());

    // dividend, sex_digits > d/6, d%6
    // 120, '' > 20, '0'
    // 20, '0' > 3, '20'
    // 3, '20' > 0, '320'

    while !dividend.iter().all(|&d| d == Zero) {
        let mut dividend_slice = &mut dividend[..];
        
        while dividend_slice.len() > 1 || dividend_slice[0] >= Six {
            // 1 2 | 2
            let first_sexgit = dividend_slice[0];
            if first_sexgit >= Six {
                dividend_slice[0] -= 6;
                quotient.push(One);
            } else {
                // '020 0'00 | '00
                dividend_slice[0] = Zero;
                // 0'20 00'0 | 0'0
                dividend_slice = &mut dividend_slice[1..];
                let (quo, rem) = digit_div_rem_lt60_6((first_sexgit, dividend_slice[0]));

                // 0'00 00'0 | 0'2
                dividend_slice[0] = rem;
                // 2 02 | 3
                quotient.push(quo);
            }
        }
        // 20 | 3
        quotient.reverse();

        // 0 | 2
        sex_digits.push(unsafe {std::mem::transmute(dividend_slice[0])});
        let quotient_len = quotient.len();
        dividend = std::mem::replace(&mut quotient, Digits::with_capacity(quotient_len));
    }
    for _ in 0..leading_zeroes {
        sex_digits.push(SeximalDigit::Zero);
    }


    sex_digits.reverse();
    sex_digits
}

#[cfg(test)]
mod tests {
    use super::{convert_to_seximal, digits_from_num, SeximalDigit::*};

    #[test]
    fn zero() {
        assert_eq!(convert_to_seximal(digits_from_num(0)), &[Zero]);
    }

    #[test]
    fn some_numbers() {
        assert_eq!(convert_to_seximal(Vec::new()), &[]);
        assert_eq!(convert_to_seximal(digits_from_num(1)), &[One]);
        assert_eq!(convert_to_seximal(digits_from_num(5)), &[Five]);
        assert_eq!(convert_to_seximal(digits_from_num(6)), &[One, Zero]);
        assert_eq!(convert_to_seximal(digits_from_num(9)), &[One, Three]);
        assert_eq!(convert_to_seximal(digits_from_num(10)), &[One, Four]);
        assert_eq!(convert_to_seximal(digits_from_num(36)), &[One, Zero, Zero]);
        assert_eq!(convert_to_seximal(digits_from_num(120)), &[Three, Two, Zero]);
        assert_eq!(convert_to_seximal(digits_from_num(124)), &[Three, Two, Four]);
    }
}
