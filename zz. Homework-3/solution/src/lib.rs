use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::io::{Error, ErrorKind};

type Row = HashMap<String, String>;
const DELIMITER: char = ',';
const QUOTE: char = '"';

pub struct Csv<R: BufRead> {
    pub columns: Vec<String>,
    reader: R,
    selection: Option<Box<dyn Fn(&Row) -> Result<bool, CsvError>>>,
}

#[derive(Debug)]
pub enum CsvError {
    IO(std::io::Error),
    ParseError(String),
    InvalidHeader(String),
    InvalidRow(String),
    InvalidColumn(String),
}

impl<R: BufRead> Csv<R> {
    pub fn new(mut reader: R) -> Result<Self, CsvError> {
        let mut headers = String::new();
        let read_bytes = reader.read_line(&mut headers);
        match read_bytes {
            Ok(0) => Err(CsvError::InvalidHeader(
                "The header row must not be empty".into(),
            )),
            Ok(_) => {
                let mut cols = Vec::<String>::new();
                let mut headers: String = headers.trim().into();
                if !headers.contains(DELIMITER) {
                    cols.push(headers.trim().into());
                } else {
                    // contains the delimiter, for sure
                    while let Some((col_name, rest)) = take_and_skip(&headers, DELIMITER) {
                        cols.push(col_name.trim().into());
                        headers = rest.into();
                    }
                    cols.push(headers.trim().into()); // the last header
                }

                let mut distinct_cols = Vec::<String>::new();
                for col in cols.clone() {
                    if distinct_cols.contains(&col) {
                        return Err(CsvError::InvalidHeader(
                            "The names of the columns must be unique!".into(),
                        ));
                    }
                    distinct_cols.push(col);
                }

                Ok(Csv {
                    columns: cols,
                    reader,
                    selection: None,
                })
            }
            Err(error) => Err(CsvError::IO(error)),
        }
    }

    pub fn parse_line(&self, line: &str) -> Result<Row, CsvError> {
        parse_row_with_cols(&self.columns, line)
    }

    pub fn apply_selection<F>(&mut self, callback: F)
    where
        F: Fn(&Row) -> Result<bool, CsvError> + 'static,
    {
        self.selection = Some(Box::new(callback));
    }

    fn gen_header_str(&self) -> String {
        let mut res = String::new();
        for i in 0..(self.columns.len() - 1) {
            res += &self.columns[i];
            res += ", "
        }
        res += &self.columns[self.columns.len() - 1];
        res + "\n"
    }

    pub fn write_to<W: Write>(self, mut writer: W) -> Result<(), CsvError> {
        let header_str = self.gen_header_str();
        let header_write_res = writer.write(header_str.as_bytes());
        match header_write_res {
            Ok(n) => {
                if n != header_str.len() {
                    return Err(CsvError::IO(Error::new(
                        ErrorKind::Other,
                        "Could not write all the bytes of the headers!",
                    )));
                }
            }
            Err(error) => return Err(CsvError::IO(error)),
        }

        let Csv {
            reader,
            columns,
            selection,
        } = self;

        let lines: Vec<_> = reader.lines().collect();

        for line in lines {
            if line.is_err() {
                return Err(CsvError::IO(Error::new(
                    ErrorKind::Other,
                    "Could not read line!",
                )));
            }
            let line = line.unwrap();
            let parse_res = parse_row_with_cols(&columns, &line);
            match parse_res {
                Ok(parsed) => {
                    let formatted_row = gen_row_str_from_cols(&columns, &parsed);
                    let should_include = match &selection {
                        Some(box_fn) => match box_fn(&parsed) {
                            Ok(should) => should,
                            Err(error) => return Err(error),
                        },
                        None => true,
                    };
                    if !should_include {
                        continue;
                    }
                    let write_res = writer.write(formatted_row.as_bytes());
                    match write_res {
                        Ok(n) => {
                            if n != formatted_row.len() {
                                return Err(CsvError::IO(Error::new(
                                    ErrorKind::Other,
                                    "Could not write all bytes of row!",
                                )));
                            }
                        }
                        Err(error) => return Err(CsvError::IO(error)),
                    }
                }
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }
}

impl<R: BufRead> Iterator for Csv<R> {
    type Item = Result<Row, CsvError>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut curr_row = String::new();
            let read_res = self.reader.read_line(&mut curr_row);
            match read_res {
                Ok(0) => return None,
                Ok(_) => match self.parse_line(&curr_row) {
                    Ok(row) => {
                        let should_return = match &self.selection {
                            Some(select_fn) => match select_fn(&row) {
                                Ok(res) => res,
                                Err(error) => return Some(Err(error)),
                            },
                            None => true,
                        };
                        if should_return {
                            return Some(Ok(row));
                        } else {
                            continue;
                        }
                    }
                    Err(error) => return Some(Err(error)),
                },
                Err(error) => return Some(Err(CsvError::IO(error))),
            }
        }
    }
}

pub fn skip_next(input: &str, target: char) -> Option<&str> {
    if input.is_empty() {
        return None;
    }

    if input.chars().nth(0).unwrap() == target {
        let mut ch_stream = input.chars();
        ch_stream.next();
        Some(&ch_stream.as_str())
    } else {
        None
    }
}

pub fn take_until(input: &str, target: char) -> (&str, &str) {
    if input.is_empty() {
        return ("", "");
    }

    let mut index = 0;
    for ch in input.chars() {
        if ch == target {
            break;
        } else {
            index += 1;
        }
    }
    let mut left_end: usize = 0;
    input
        .chars()
        .into_iter()
        .take(index)
        .for_each(|x| left_end += x.len_utf8());
    let left_side = &input[..left_end];
    let right_side = &input[left_end..];

    (&left_side, &right_side)
}

pub fn take_and_skip(input: &str, target: char) -> Option<(&str, &str)> {
    if input.is_empty() || !input.contains(target) {
        return None;
    }

    let mut split = take_until(input, target);
    split.1 = skip_next(split.1, target).unwrap();

    Some(split)
}

fn parse_row_with_cols(columns: &Vec<String>, line: &str) -> Result<Row, CsvError> {
    let mut row: Row = Row::new();
    let mut line = line.trim();

    match skip_next(&line, QUOTE) {
        Some(res) => {
            line = res;
            for col in columns {
                if line.is_empty() {
                    return Err(CsvError::ParseError(
                        "There should be a value for every column!".into(),
                    ));
                }
                let curr_value = take_and_skip(&line, QUOTE);
                match curr_value {
                    Some((col_val, rest)) => {
                        row.insert(col.into(), col_val.into());
                        line = rest;
                    }
                    None => {
                        return Err(CsvError::ParseError(
                            "Every opening quote should have a matching closed one!".into(),
                        ));
                    }
                }
                line = line.trim();
                let rest = skip_next(&line, DELIMITER);
                match rest {
                    Some(rest) => line = rest,
                    None => line = "",
                }
                line = skip_next(line.trim(), QUOTE).unwrap_or_else(|| &"");
            }
            if !line.is_empty() {
                return Err(CsvError::ParseError("There should be no extra information in the row other than a value for each column!".into()));
            }
            Ok(row)
        }
        None => Err(CsvError::ParseError(
            "Values need to be encased in quotes".into(),
        )),
    }
}

fn gen_row_str_from_cols(columns: &Vec<String>, row: &Row) -> String {
    let mut res = String::new();
    for i in 0..(columns.len() - 1) {
        res += &format!("\"{}\"", &row[&columns[i]]);
        res += ", ";
    }
    res += &format!("\"{}\"", &row[&columns[columns.len() - 1]]);
    res + "\n"
}
