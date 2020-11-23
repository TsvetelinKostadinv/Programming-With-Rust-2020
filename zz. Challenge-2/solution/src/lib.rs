use std::borrow::Cow;
#[derive(Clone)]
pub struct FizzBuzzer {
    labels: [String; 3],
}
impl FizzBuzzer {
    pub fn new(labels: [String; 3]) -> Self {
        FizzBuzzer { labels }
    }

    pub fn iter(&self) -> FizzBuzzerIter {
        FizzBuzzerIter {
            fizzbuzzer: self,
            current_number: 0 as usize,
        }
    }
}

#[derive(Clone)]
pub struct FizzBuzzerIter<'a> {
    fizzbuzzer: &'a FizzBuzzer,
    current_number: usize,
}

impl<'a> Iterator for FizzBuzzerIter<'a> {
    type Item = Cow<'a, str>;
    /// Очакваме всяко извикване на тази функция да връща следващото естествено число, започващо от
    /// 1:
    ///
    /// - Ако числото се дели на 3, връщаме `Cow::Borrowed`, държащо reference към `labels[0]`
    /// - Ако числото се дели на 5, връщаме `Cow::Borrowed`, държащо reference към `labels[1]`
    /// - Ако числото се дели на 15, връщаме `Cow::Borrowed`, държащо reference към `labels[2]`
    /// - Иначе, връщаме `Cow::Owned`, държащо числото, конвертирано до `String`
    ///
    fn next(&mut self) -> Option<Self::Item> {
        // Some(Cow::Borrowed(&self.fizzbuzzer.labels[0]));
        self.current_number += 1;
        match self.current_number {
            fizzbuzz if fizzbuzz % 15 == 0 => Some(Cow::Borrowed(&self.fizzbuzzer.labels[2])),
            fizz if fizz % 3 == 0 => Some(Cow::Borrowed(&self.fizzbuzzer.labels[0])),
            buzz if buzz % 5 == 0 => Some(Cow::Borrowed(&self.fizzbuzzer.labels[1])),
            n => Some(Cow::Owned(n.to_string())),
        }
    }
}
