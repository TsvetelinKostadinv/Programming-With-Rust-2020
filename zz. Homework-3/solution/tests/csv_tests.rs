use solution::*;
use std::io::{self, BufRead, BufReader, Read};

struct ErroringReader {}

impl Read for ErroringReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "read error!"))
    }
}

impl BufRead for ErroringReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::new(io::ErrorKind::Other, "fill_buf error!"))
    }

    fn consume(&mut self, _amt: usize) {}
}

#[test]
fn construction_test_normal_construction() {
    let data = r#"
        name, age, birth date
        "Gen Z. Person", "20", "2000-01-01"
    "#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();
    println!("{:?}", csv.columns);
}

#[test]
#[should_panic]
fn construction_test_diplicate_cols() {
    let data = r#"
        name, age, birth date, age
        "Gen Z. Person", "20", "2000-01-01"
    "#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();
    println!("{:?}", csv.columns);
}

#[test]
#[should_panic]
fn construction_test_empty_string() {
    let data = "".trim().as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();
    println!("{:?}", csv.columns);
}

#[test]
#[should_panic]
fn construction_test_io_error() {
    let data = ErroringReader {};

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();
    println!("{:?}", csv.columns);
}
