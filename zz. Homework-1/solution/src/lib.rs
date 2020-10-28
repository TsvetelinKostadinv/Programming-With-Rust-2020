/// Generates a sequence from one to the inputted number
fn gen_sequence_one_to(n: usize) -> Vec<usize> {
    (1..n + 1).collect()
}

/// Transorms the given num according to the rule
/// If it is devisible by k1 then lable_k1 is returned
/// If it is devisible by k2 then lable_k2 is returned
/// If it is devisible by both then label_both is returned
/// Also the type casting is taken care of in here so nowhere else do you need to do it
fn transform_for_keys(
    num: usize,
    k1: u8,
    label_k1: String,
    k2: u8,
    label_k2: String,
    label_both: String,
) -> String {
    match num {
        _ if num % (k1 as usize) == 0 && num % (k2 as usize) == 0 => label_both,
        _ if num % (k1 as usize) == 0 => label_k1,
        _ if num % (k2 as usize) == 0 => label_k2,
        _ => num.to_string(),
    }
}

/// A specialization of the transform_for_keys function to be appplicable to the normal game of FizzBuzz
fn fizz_buzz_transformer(num: usize) -> String {
    transform_for_keys(
        num,
        3,
        "Fizz".to_string(),
        5,
        "Buzz".to_string(),
        "FizzBuzz".to_string(),
    )
}

/// A specialization of the transform_for_keys function to be appplicable to the normal game of FizzBuzz
fn custom_buzz_transformer(num: usize, k1: u8, k2: u8) -> String {
    transform_for_keys(
        num,
        k1,
        "Fizz".to_string(),
        k2,
        "Buzz".to_string(),
        "FizzBuzz".to_string(),
    )
}

/// Generates elements according to the standard rules of the fizzbuzz game
pub fn fizzbuzz(n: usize) -> Vec<String> {
    gen_sequence_one_to(n)
        .iter()
        .map(|&num| fizz_buzz_transformer(num))
        .collect()
}

/// Genereates elements according to the rules of the fizzbuzz games, but with custom coefficients
pub fn custom_buzz(n: usize, k1: u8, k2: u8) -> Vec<String> {
    gen_sequence_one_to(n)
        .iter()
        .map(|&num| custom_buzz_transformer(num, k1, k2))
        .collect()
}

/// Defines a structure to geenrate elements 
/// according to the rules of the game FizzBuzz, 
/// but with custom constants and custom labels
pub struct FizzBuzzer {
    pub k1: u8,
    pub k2: u8,
    pub labels: [String; 3],
}

impl FizzBuzzer {
    /// Takes the first n elements which will be 
    /// generated according to the rules of the game fizzbuzz,
    /// but specified in the structure
    pub fn take(&self, n: usize) -> Vec<String> {
        gen_sequence_one_to(n)
            .iter()
            .map(|&num| {
                return transform_for_keys(
                    num,
                    self.k1,
                    self.labels[0].clone(),
                    self.k2,
                    self.labels[1].clone(),
                    self.labels[2].clone(),
                );
            })
            .collect()
    }

    /// Changes the labels at the specified index
    pub fn change_label(&mut self, index: usize, value: &String) {
        self.labels[index] = value.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generator_test() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(gen_sequence_one_to(10), expected);
    }

    #[test]
    fn fizz_buzz_transformer_test() {
        assert_eq!(fizz_buzz_transformer(1), 1.to_string());
        assert_eq!(fizz_buzz_transformer(2), 2.to_string());
        assert_eq!(fizz_buzz_transformer(3), "Fizz");
        assert_eq!(fizz_buzz_transformer(5), "Buzz");
        assert_eq!(fizz_buzz_transformer(15), "FizzBuzz");
    }

    #[test]
    fn same_behaviour_test() {
        fn custom_fizz(n: usize) -> Vec<String> {
            custom_buzz(n, 3, 5)
        }
        let fizzbuzzer = FizzBuzzer {
            k1: 3,
            k2: 5,
            labels: [
                "Fizz".to_string(),
                "Buzz".to_string(),
                "FizzBuzz".to_string(),
            ],
        };
        for i in 1..20 {
            assert_eq!(fizzbuzz(i), custom_fizz(i));
            assert_eq!(fizzbuzz(i), fizzbuzzer.take(i));
        }
    }

    #[test]
    fn empty_vector_test() {
        let empty: Vec<String> = Vec::new();
        let fizzbuzzer = FizzBuzzer {
            k1: 3,
            k2: 5,
            labels: [
                "Fizz".to_string(),
                "Buzz".to_string(),
                "FizzBuzz".to_string(),
            ],
        };
        assert_eq!(fizzbuzz(0), empty);
        assert_eq!(custom_buzz(0, 3, 5), empty);
        assert_eq!(fizzbuzzer.take(0), empty);
    }
}
