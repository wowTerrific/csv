use super::{Record, errors, Result};


// TODO: make sure anything within "" is contained within a field...
pub fn raw_csv_to_records(raw: &str) -> Result<Vec<Record>> {
    if !raw.contains(',') || !raw.contains('\n') {
        return Err(
            Box::new(errors::ParseError {
                message: String::from("Unable to parse CSV, please check file"),
            })
        );
    }

    let mut data: Vec<Record> = Vec::new();
    dbg!(&raw);

    // TODO: Check first line for 'sep=<char>' and use
    // that as the split!


    // NEED TO ADD LINE-STATE as boolean to accomade
    // the use of newline characters within freaking
    // quotations dude. - 
    // maybe add a check to see if the number of '"' characters is odd to
    // do a 'manual' append instead of `parse_string_to_record`
    let mut in_quotes_state = false;
    for line in raw.lines() {               // FIX THIS SHIT

        let append_record = parse_string_to_record(line);

        if line.matches("\"").count() % 2 != 0 {
            if in_quotes_state && data.len() > 0 {
                let working_record_index = data.len() - 1;
                let working_record = &mut data[working_record_index];



                /****** HERE IS WHERE I STOPPED!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! *********************************************************/
                /******** IMPORTANT NOTE - DON'T USE THE RAW.LINES() METHOD. CREATE A HELPER FUNCTION TO PARSE INTO LINES THEN
                 * PARSE THE INDIVIDUAL LINES INTO RECORDS!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                 */
                // add append_record to working_record
                let rest_of_the_line = &append_record[0];

                continue; // next line please
            }
            in_quotes_state = !in_quotes_state;
        } 

       data.push(append_record);        
        
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

// TODO: delimeter should be variable instead of a hard-coded ','
fn parse_string_to_record(raw_string: &str) -> Record {
    let mut record: Record = Vec::new();
    let mut in_quotes = false;
    let mut current = String::new();

    for c in raw_string.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
            continue;
        } else if c == ',' && !in_quotes {
            if current.len() > 0 {
                record.push(current);
                current = String::new();
                continue;
            }
        }

        current.push(c);
    }

    if current.len() > 0 {
        record.push(current);
    }

    record
}


#[cfg(test)]
mod utils_tests {
    use crate::Record;
    use super::*;

    #[test]
    fn test_parse_string_to_record() {
        let ut = "\"one,one,one\",two,three";
        let expected: Record = vec![
            String::from("one,one,one"), 
            String::from("two"), 
            String::from("three")
        ];

        assert_eq!(expected, parse_string_to_record(ut));
    }

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
}
