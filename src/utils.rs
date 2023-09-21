use super::{Record, errors, Result};

/// In it's curernt form, this will remove all `\r` characters. as
/// new csv lines are denoted by the `\n` character.
pub fn raw_csv_to_records(raw: &str) -> Result<Vec<Record>> {
    if !raw.contains(',') || !raw.contains('\n') {
        return Err(
            Box::new(errors::ParseError {
                message: String::from("Unable to parse CSV, please check file"),
            })
        );
    }
    // TODO: Check first line for 'sep=<char>' and use
    // that as the second delimeter!

    let lines: Vec<String> = parse_string_to_vec_ignore_quotes(raw, '\n', false);
    let mut data: Vec<Record> = Vec::new();

    for line in lines {
        let record = parse_string_to_vec_ignore_quotes(&line, ',', true);
        data.push(record);
    }

    Ok(data)
}


// TODO: add method to check for ',' in record fields, if so, 
// surround that field with quotation marks
pub fn records_to_string(records: &Vec<Record>, c: char) -> String {
    let mut combined_records: Vec<String> = Vec::new();
    if c != ',' {
        let separtor_line = format!("sep={}\n", c);
        combined_records.push(separtor_line);
    }
    

    for record in records {
        let mut record_string = String::new();
        for item in record {
            record_string.push_str(item);
            record_string.push(c);
        }

        // removes the last ","
        record_string.pop();
        record_string.push('\n');
        combined_records.push(record_string);
    }

    let mut result_string = combined_records.into_iter().collect::<String>();
    result_string.pop(); // removes last "\n" character
    result_string
}


pub fn path_validate(path: &str) {
    if !path.ends_with(".csv") {
        panic!("Path does not point to a CSV file. Please check your CSV instance...")
    }
}

/// Private function but important to know that this will remove `\r` characters.
fn parse_string_to_vec_ignore_quotes(raw_string: &str, delimeter: char, result_as_record: bool) -> Vec<String> {
    let mut return_vec = Vec::new();
    let mut in_quotes = false;
    let mut current = String::new();

    for c in raw_string.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
            if result_as_record {
                continue;
            }
        } else if c == delimeter && !in_quotes {
            return_vec.push(current);
            current = String::new();
            continue;
        } else if c == '\r' {
            continue;
        }

        current.push(c);
    }

    return_vec.push(current);

    return_vec
}


#[cfg(test)]
mod utils_tests {
    use crate::Record;
    use super::*;

    #[test]
    fn utils_records_to_string() {
        let records: Vec<Record> = vec![
            vec![String::from("one"), String::from("two"), String::from("three")],
            vec![String::from("four"), String::from("five"), String::from("six")],
            vec![String::from("seven"), String::from("eight"), String::from("nine")],
        ];

        let result = records_to_string(&records, ',');
        assert_eq!(String::from("one,two,three\nfour,five,six\nseven,eight,nine"), result);
    }

    #[test]
    fn utils_records_to_string_with_blanks() {
        let records: Vec<Record> = vec![
            vec![String::from("one"), "".to_string(), String::from("three")],
            vec![String::from("four"), String::from("five"), String::from("six")],
            vec![String::from("seven"), String::from("eight"), String::from("nine")],
        ];

        let result = records_to_string(&records, ',');
        assert_eq!(String::from("one,,three\nfour,five,six\nseven,eight,nine"), result);
    }

    #[test]
    fn utils_raw_csv_to_records() {
        let expected: Vec<Record> = vec![
            vec![String::from("one"), String::from("two"), String::from("three")],
            vec![String::from("four"), String::from("five"), String::from("six")],
            vec![String::from("seven"), String::from("eight"), String::from("nine")],
        ];
        let csv_string = String::from("one,two,three\nfour,five,six\nseven,eight,nine");

        let result = raw_csv_to_records(&csv_string).unwrap_or(vec![vec!["FAIL".to_string()]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn utils_parse_csv_with_quotes_newlines_into_records() {
        let expected: Vec<Record> = vec![
            vec![String::from("one,one,one"), String::from("two"), String::from("three")],
            vec![String::from("four"), String::from("fi\nve"), String::from("six")],
            vec![String::from("s,e,v\ne,n"), String::from("eight"), String::from("nine")],
        ];

        let csv_string = String::from("\"one,one,one\",two,three\nfour,\"fi\nve\",six\n\"s,e,v\ne,n\",eight,nine");
        let result = raw_csv_to_records(&csv_string).expect("failed to parse quotes and newlines into to CSV");
        assert_eq!(expected, result);
    }

    #[test]
    fn create_lines_with_string_to_vec_ignore_quotes() {
        let expected: Vec<String> = vec![
            String::from("\"one,one,one\",two,three"),
            String::from("four,\"fi\nve\",six"),
            String::from("\"s,e,v\ne,n\",eight,nine"),
        ];

        let raw_data = String::from("\"one,one,one\",two,three\nfour,\"fi\nve\",six\n\"s,e,v\ne,n\",eight,nine");
        let result = parse_string_to_vec_ignore_quotes(&raw_data, '\n', false);
        assert_eq!(expected, result);
    }

    #[test]
    fn create_records_with_string_to_vec_ignore_quotes() {
        let expected: Vec<String> = vec![
            String::from("s,e,v\ne,n"),
            String::from("eight"),
            String::from("nine"),
        ];

        let raw_data = String::from("\"s,e,v\ne,n\",eight,nine");
        let result = parse_string_to_vec_ignore_quotes(&raw_data, ',', true);
        assert_eq!(expected, result);
    }

    #[test]
    fn create_records_with_empty_strings_to_vec() {
        let expected: Vec<String> = vec![
            String::from(""),
            String::from("s,e,v\ne,n"),
            String::from(""),
            String::from("nine"),
        ];

        let raw_data = String::from(",\"s,e,v\ne,n\",,nine");
        let result = parse_string_to_vec_ignore_quotes(&raw_data, ',', true);
        assert_eq!(expected, result);
    }
}
