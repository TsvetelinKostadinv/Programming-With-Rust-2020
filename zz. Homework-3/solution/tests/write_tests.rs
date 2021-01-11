use solution::*;

use std::io::BufReader;

#[test]
fn write_test() {
    let data = r#"
    name,  age    ,birth date
    "Douglas Adams","42","1952-03-11"
    "Gen Z. Person",    "20"   ,   "2000-01-01"
    "Ada Lovelace","36","1815-12-10"
"#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();

    let mut output = Vec::new();
    csv.write_to(&mut output).unwrap();

    println!("{}", String::from_utf8(output).unwrap());
}
