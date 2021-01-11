use solution::*;
use std::io::BufReader;

#[test]
fn iter_test() {
    let reader = BufReader::new(
        r#"
    name, age, birth date
    "Douglas Adams", "42", "1952-03-11"
    "Gen Z. Person", "20", "2000-01-01"
    "Ada Lovelace", "36", "1815-12-10"
"#
        .trim()
        .as_bytes(),
    );

    // Конструираме си CSV-то:
    let mut csv = Csv::new(reader).unwrap();

    // Инсталираме условието -- само редове с възраст над 30 ще останат:
    csv.apply_selection(|row| {
        let age = row
            .get("age")
            .ok_or_else(|| CsvError::InvalidColumn(String::from("age")))?;
        let age = age
            .parse::<u32>()
            .map_err(|_| CsvError::ParseError(String::from(age)))?;

        Ok(age > 30)
    });

    // Итерираме през резултата:
    while let Some(Ok(row)) = csv.next() {
        println!("{:?}", row["name"]);
    }
}
