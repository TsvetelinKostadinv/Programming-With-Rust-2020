use solution::*;
use std::io::BufReader;

#[test]
fn parse_right_row_test() {
    let data = r#"
    name, age, birth date
"#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();

    println!("{:?}", csv.columns);
    let res = csv
        .parse_line(r##"     "Some name" ,     "123","12.03.2001"       "##)
        .unwrap();

    assert_eq!(res["name"], "Some name");
    assert_eq!(res["age"], "123");
    assert_eq!(res["birth date"], "12.03.2001");
}

#[test]
#[should_panic]
fn parse_too_long_row_test() {
    let data = r#"
    name, age, birth date
"#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();

    println!("{:?}", csv.columns);
    let res = csv
        .parse_line(r##"     "Some name" ,     "123","12.03.2001", "asd""##)
        .unwrap();

    assert_eq!(res["name"], "Some name");
    assert_eq!(res["age"], "123");
    assert_eq!(res["birth date"], "12.03.2001");
}

#[test]
#[should_panic]
fn parse_no_quotes_row_test() {
    let data = r#"
    name, age, birth date
"#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();

    println!("{:?}", csv.columns);
    let res = csv
        .parse_line(r##"     "Some name" ,     123,"12.03.2001""##)
        .unwrap();

    assert_eq!(res["name"], "Some name");
    assert_eq!(res["age"], "123");
    assert_eq!(res["birth date"], "12.03.2001");
}

#[test]
#[should_panic]
fn parse_too_short_row_test() {
    let data = r#"
    name, age, birth date
"#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();

    println!("{:?}", csv.columns);
    let res = csv.parse_line(r##"     "Some name" ,     "123""##).unwrap();

    assert_eq!(res["name"], "Some name");
    assert_eq!(res["age"], "123");
    assert_eq!(res["birth date"], "12.03.2001");
}

#[test]
fn parse_right_row_with_line_break_test() {
    let data = r#"
    name, age, birth date
"#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();

    println!("{:?}", csv.columns);
    let res = csv
        .parse_line(
            r##"     "Some name" ,     "123","12.03.2001"    
          "##,
        )
        .unwrap();

    assert_eq!(res["name"], "Some name");
    assert_eq!(res["age"], "123");
    assert_eq!(res["birth date"], "12.03.2001");
}

#[test]
fn parse_right_row_with_extra_comma() {
    let data = r#"
    name, age, birth date
    "Douglas Adams", "42", "1952-03-11"
    "#
    .trim()
    .as_bytes();

    let csv = Csv::new(BufReader::new(data));

    let csv = csv.unwrap();

    println!("{:?}", csv.columns);
    let res = csv
        .parse_line(
            r##"
            "Ada, Countess of Lovelace", "36", "1815-12-10"
            "##,
        )
        .unwrap();

    assert_eq!(res["name"], "Ada, Countess of Lovelace");
    assert_eq!(res["age"], "36");
    assert_eq!(res["birth date"], "1815-12-10");
}
