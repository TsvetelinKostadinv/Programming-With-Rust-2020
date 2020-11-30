use std::{cmp::Ordering, fmt::Display, ops::Add, ops::Neg, ops::Sub, str::FromStr};
// at the bottom of the file there are tests,
// which are disbled, they are very slow and
// I didn't know if should include them

/// Represents a sign of a number +, - or none
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Sign {
    Positive,
    Negative,
    None,
}

impl Sign {
    fn negate(&self) -> Self {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
            Sign::None => Sign::None,
        }
    }
}

/// This is a theoretically infinite precision integral value
/// @apiNote contains a sign and a vector of the digits, which are in reverse order
/// so 123 would be represented as a minus sign and 3,2,1 in the vector
/// this makes addition/subtraction of numbers easier to implement\
/// \
/// IMPORTANT: 0 is represented as a number without a sign and the digit 0
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bigint {
    sign: Sign,
    digits: Vec<u8>,
}

impl Bigint {
    /// Constructs a zero
    pub fn new() -> Self {
        Bigint {
            sign: Sign::None,
            digits: vec![0],
        }
    }

    /// A new bigint from the digits and a sign. NOT public - utility only
    fn from_components(digits: Vec<u8>, sign: Sign) -> Self {
        let mut significant = Vec::<u8>::new();
        let mut leading_zero = true;
        for digit in digits {
            match digit {
                0 => {
                    if !leading_zero {
                        significant.insert(0, 0);
                    }
                }
                n => {
                    if n > 9 {
                        panic!(
                            String::from("A single digit cannot be greater than 9: ")
                                + n.to_string().as_str()
                        );
                    }
                    leading_zero = false;
                    significant.insert(0, n);
                }
            }
        }

        if significant.is_empty() {
            Bigint::new()
        } else {
            Bigint {
                digits: significant,
                sign,
            }
        }
    }

    /// Pretty much self explanatory, but\
    /// Returns true if the number is positive, false if it is negative or zero
    pub fn is_positive(&self) -> bool {
        match self.sign {
            Sign::Positive => true,
            _ => false,
        }
    }

    /// Pretty much self explanatory, but\
    /// Returns true if it is negative, false if the number is positive or zero
    pub fn is_negative(&self) -> bool {
        match self.sign {
            Sign::Negative => true,
            _ => false,
        }
    }

    /// Returns the absolute value of this integer
    pub fn abs(&self) -> Bigint {
        Bigint {
            digits: self.digits.clone(),
            sign: Sign::Positive,
        }
    }
}

impl Neg for &Bigint {
    type Output = Bigint;

    fn neg(self) -> Self::Output {
        Bigint {
            digits: self.digits.clone(),
            sign: self.sign.negate(),
        }
    }
}

impl From<i32> for Bigint {
    fn from(val: i32) -> Self {
        let negative = match val {
            0 => Sign::None,
            n if n > 0 => Sign::Positive,
            n if n < 0 => Sign::Negative,
            _ => unreachable!("i32 is either less than, greater than or 0"),
        };
        let mut digits = Vec::<u8>::new();
        let mut copy = val.abs() as usize;
        while copy > 0 {
            digits.push((copy % 10) as u8);
            copy /= 10;
        }
        digits.reverse();

        Bigint::from_components(digits, negative)
    }
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Bigint {
    type Err = ParseError;

    /// The string should contain the base-10 digits and (optionally) a sign
    ///
    /// Bigint::from_str("123"); // => positive by default
    /// Bigint::from_str("+123");
    /// Bigint::from_str("-123");
    ///
    /// Don't forget zero is neither positive nor negative, so the sign is ignored
    ///
    /// "" == 0
    ///
    /// Leading zeros are ignored
    ///
    /// If any other sign is inputted, a parse error is emmitted
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Ok(Bigint::new());
        }
        let potential_sign = s.chars().nth(0).unwrap(); // unwrap is safe, because the if-check before ensures that s has at least one char
        let skip_char = if potential_sign == '-' || potential_sign == '+' {
            1
        } else {
            0
        };
        let contains_only_digits = s.chars().skip(skip_char).all(|ch| match ch {
            // NOTE: could have used char::is_numeric or char::is_digit, but it matches other types of chars also
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,
            _ => false,
        });

        if !contains_only_digits {
            return Err(ParseError);
        }

        let significant: Vec<_> = s
            .chars()
            .into_iter()
            .skip(skip_char)
            .skip_while(|&ch| ch == '0')
            .collect();

        let sign = match potential_sign {
            '+' => Sign::Positive,
            '-' => Sign::Negative,
            _ => {
                if significant.is_empty() {
                    Sign::None
                } else {
                    Sign::Positive
                }
            }
        };

        Ok(Bigint::from_components(
            significant
                .into_iter()
                .map(|digit| digit.to_digit(10).unwrap() as u8)
                .collect(),
            sign,
        ))
    }
}

impl PartialOrd for Bigint {
    /// These are integer numbers so they can always be compared
    fn partial_cmp(&self, other: &Bigint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bigint {
    fn cmp(&self, other: &Bigint) -> Ordering {
        if self.sign == Sign::Positive && other.sign == Sign::Negative {
            return Ordering::Greater;
        } else if self.sign == Sign::Negative && other.sign == Sign::Positive {
            return Ordering::Less;
        } else if self.sign == Sign::None && other.sign == Sign::None {
            return Ordering::Equal;
        } else if self.sign == Sign::None && other.sign == Sign::Negative {
            return Ordering::Greater;
        } else if self.sign == Sign::None && other.sign == Sign::Positive {
            return Ordering::Less;
        } else if self.sign == Sign::Positive && other.sign == Sign::None {
            return Ordering::Greater;
        } else if self.sign == Sign::Negative && other.sign == Sign::None {
            return Ordering::Less;
        }
        // the signs are the same

        if self.sign == other.sign && self.sign == Sign::Negative {
            return (-self).cmp(&-other).reverse();
        }
        // both signs are positive

        if self.digits.len() != other.digits.len() {
            return self.digits.len().cmp(&other.digits.len());
        }
        // they have the same number of digits and are different
        if self.digits == other.digits {
            return Ordering::Equal;
        }

        for i in (0..self.digits.len()).rev() {
            let self_digit = *self.digits.get(i).unwrap();
            let other_digit = *other.digits.get(i).unwrap();
            if self_digit > other_digit {
                return Ordering::Greater;
            } else if self_digit < other_digit {
                return Ordering::Less;
            }
        }
        println!(
            "Could not compare: {:?} and {:?}",
            self.digits, other.digits
        );
        unreachable!("The numbers are not equal, but they don't have different digits????");
    }
}

impl Display for Bigint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let number = self
            .digits
            .iter()
            .rev()
            .map(|digit| digit.to_string())
            .collect::<Vec<String>>()
            .join("");
        match f.write_str(if self.sign == Sign::Negative { "-" } else { "" }) {
            Ok(_) => f.write_str(&number),
            err => err,
        }
    }
}

impl Add for Bigint {
    type Output = Bigint;

    fn add(self, other: Self) -> Self {
        if self.sign == Sign::Positive && other.sign == Sign::Positive {
            let len = if self.digits.len() > other.digits.len() {
                self.digits.len()
            } else {
                other.digits.len()
            };
            let mut carry = 0_u8;
            let mut res_digits = Vec::<u8>::new();
            for i in 0..=len {
                let self_digit = self.digits.get(i).unwrap_or(&0);
                let other_digit = other.digits.get(i).unwrap_or(&0);
                let mut digit = carry + self_digit + other_digit;
                carry = 0;
                if digit >= 10 {
                    carry = digit / 10;
                    digit %= 10;
                }
                res_digits.insert(0, digit);
            }

            return Bigint::from_components(res_digits, Sign::Positive);
        } else if self.sign == Sign::Negative && other.sign == Sign::Negative {
            return -&(self.abs() + other.abs());
        } else if self.sign == Sign::Positive && other.sign == Sign::Negative {
            return self - other.abs();
        } else if self.sign == Sign::Negative && other.sign == Sign::Positive {
            return other - self.abs();
        } else if self.sign == Sign::None {
            return other.clone();
        } else if other.sign == Sign::None {
            return self.clone();
        }
        println!("Failed to add {:?} and {:?}", self, other);

        unreachable!()
    }
}

impl Sub for Bigint {
    type Output = Bigint;
    fn sub(self, other: Self) -> Self {
        if self.sign == Sign::Positive && other.sign == Sign::Positive {
            if self < other {
                return -&(other - self);
            }
            let len = if self.digits.len() > other.digits.len() {
                self.digits.len()
            } else {
                other.digits.len()
            };
            let mut carry = 0_i8;
            let mut res_digits = Vec::<u8>::new();
            for i in 0..=len {
                let self_digit = *self.digits.get(i).unwrap_or(&0) as i8;
                let other_digit = *other.digits.get(i).unwrap_or(&0) as i8;
                let mut digit = self_digit - other_digit + carry;
                carry = 0;
                if digit < 0 {
                    carry = -1;
                    digit += 10;
                }
                res_digits.insert(0, digit as u8);
            }
            return Bigint::from_components(res_digits, Sign::Positive);
        } else if self.sign == Sign::Negative && other.sign == Sign::Negative {
            return -&(self.abs() + other.abs());
        } else if self.sign == Sign::Positive && other.sign == Sign::Negative {
            return self - other.abs();
        } else if self.sign == Sign::Negative && other.sign == Sign::Positive {
            return other - self.abs();
        } else if self.sign == Sign::None {
            return -&other;
        } else if other.sign == Sign::None {
            return self.clone();
        }
        println!("Failed to subtract {:?} and {:?}", self, other);

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_components_zero_test() {
        let real_zero = Bigint::new();
        let pos_zero = Bigint::from_components(vec![0, 0, 0], Sign::Positive);
        let neg_zero = Bigint::from_components(vec![0, 0], Sign::Negative);

        assert_eq!(real_zero, pos_zero);
        assert_eq!(real_zero, neg_zero);
        assert_eq!(pos_zero, neg_zero);
    }

    #[test]
    fn from_components_test() {
        let zero = Bigint::new();
        let num123 = Bigint::from_components(vec![1, 2, 3], Sign::Positive);
        let another_num123 = Bigint::from_components(vec![1, 2, 3], Sign::Positive);
        let num_neg123 = Bigint::from_components(vec![1, 2, 3], Sign::Negative);

        assert_ne!(zero, num123);
        assert_ne!(zero, another_num123);
        assert_ne!(zero, num_neg123);

        assert_eq!(num123, another_num123);
        assert_ne!(num123, num_neg123);
    }

    #[test]
    fn sign_zero_test() {
        let real_zero = Bigint::new();
        let pos_zero = Bigint::from_components(vec![0, 0, 0], Sign::Positive);
        let neg_zero = Bigint::from_components(vec![0, 0], Sign::Negative);

        assert_eq!(real_zero.is_negative(), false);
        assert_eq!(real_zero.is_positive(), false);

        assert_eq!(pos_zero.is_negative(), false);
        assert_eq!(pos_zero.is_positive(), false);

        assert_eq!(neg_zero.is_negative(), false);
        assert_eq!(neg_zero.is_positive(), false);
    }

    #[test]
    fn construction_from_string_zero() {
        let zero = Bigint::new();
        assert_eq!(Bigint::from_str("+0").unwrap(), zero);
        assert_eq!(Bigint::from_str("-0").unwrap(), zero);
        assert_eq!(Bigint::from_str("0").unwrap(), zero);
        assert_eq!(Bigint::from_str("0000").unwrap(), zero);
        assert_eq!(Bigint::from_str("").unwrap(), zero);
    }

    #[test]
    fn construction_from_string_letters_err() {
        assert!(Bigint::from_str("+asd").is_err());
        assert!(Bigint::from_str("-0123a").is_err());
        assert!(Bigint::from_str("ala bala").is_err());
    }

    #[test]
    fn construction_from_str_valid() {
        for i in -1024..1024 {
            assert_eq!(
                Bigint::from(i),
                Bigint::from_str(i.to_string().as_str()).unwrap()
            );
        }
    }

    #[test]
    fn ordering_test() {
        let num123 = Bigint::from_str("123").unwrap();
        let num_neg123 = Bigint::from_str("-123").unwrap();
        let num321 = Bigint::from_str("321").unwrap();
        let num_neg321 = Bigint::from_str("-321").unwrap();

        assert!(num123 > num_neg123);
        assert!(num123 == -&num_neg123);

        assert!(num321 > num_neg321);
        assert!(num321 == -&num_neg321);

        assert!(num321 > num123);
        assert!(num_neg321 < num_neg123);
    }

    #[ignore = "slow"]
    #[test]
    fn ordering_full_test() {
        for i in -1024..1024 {
            for j in -1024..1024 {
                assert_eq!(i.cmp(&j), Bigint::from(i).cmp(&Bigint::from(j)));
            }
        }
    }

    #[test]
    fn carry_over_test() {
        assert_eq!(
            Bigint::from(1) + Bigint::from(9),
            Bigint::from_str("10").unwrap()
        );
        assert_eq!(
            Bigint::from(9) + Bigint::from(1),
            Bigint::from_str("10").unwrap()
        );
    }

    #[test]
    fn one_digit_plus_two_digit() {
        assert_eq!(
            Bigint::from(1) + Bigint::from(10),
            Bigint::from_str("11").unwrap()
        );
        assert_eq!(
            Bigint::from(10) + Bigint::from(1),
            Bigint::from_str("11").unwrap()
        );
    }

    #[ignore = "slow"]
    #[test]
    fn add_positive_test() {
        for i in 0..1024 {
            for j in 0..1024 {
                assert_eq!(Bigint::from(i + j), Bigint::from(i) + Bigint::from(j));
            }
        }
    }

    #[ignore = "slow"]
    #[test]
    fn add_test() {
        for i in -1024..1024 {
            for j in -1024..1024 {
                assert_eq!(Bigint::from(i + j), Bigint::from(i) + Bigint::from(j));
            }
        }
    }

    #[ignore = "display_test"]
    #[test]
    fn display_test() {
        for i in -1024..1024 {
            println!("{}", Bigint::from(i));
        }
    }
}
