use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

fn char_for(digit: u8) -> char {
    match digit {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => '?',
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bigint {
    pub digits: Vec<u8>,
}

impl FromStr for Bigint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());

        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                digits.push(digit as u8);
            } else {
                return Err("Invalid input!");
            }
        }

        Ok(Bigint { digits })
    }
}

impl fmt::Display for Bigint {
    /// Форматира число като за хора, по най-простия начин -- просто показва цифрите една след
    /// друга:
    ///
    ///   let bigint = Bigint::from_str("100000").unwrap();
    ///   println!("{}", bigint);
    ///   // => 100000
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for &digit in &self.digits {
            res.push(char_for(digit));
        }
        println!("{}", res);
        f.write_str(&res)
    }
}

pub struct Delimited<'a> {
    bigint: &'a Bigint,
}

impl Bigint {
    pub fn delimited(&self) -> Delimited {
        Delimited { bigint: self }
    }
}

impl<'a> fmt::Display for Delimited<'a> {
    /// Форматира Bigint по малко по-човешки начин -- със запетайки на всеки 3 цифри (за да отделим хиляди, милиони и т.н.):
    ///
    ///   let bigint = Bigint::from_str("100000").unwrap();
    ///   println!("{}", bigint.delimited());
    ///   // => 100,000
    ///
    ///   let bigint = Bigint::from_str("100000000").unwrap();
    ///   println!("{}", bigint.delimited());
    ///   // => 100,000,000
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut counter = 0;
        let mut res = String::new();
        for &digit in self.bigint.digits.iter().rev() {
            if counter == 3
            {
                res.push_str(",");
                counter = 0;
            }
            counter+=1;
            res.push(char_for(digit));
        }
        f.write_str(&res.chars().rev().collect::<String>())
    }
}

fn main() {
    let bigint = Bigint::from_str("100000").unwrap();
    println!("{}", bigint); // => 100000
    println!("{}", bigint.delimited()); // => 100,000
}
