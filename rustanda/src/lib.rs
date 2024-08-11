// src/lib.rs

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use std::any::Any;

pub fn read_csv(file_path: &str) -> Result<HashMap<String, Vec<ParsedValue>>, io::Error> {
    // Check if file is fine
    let check: Result<bool, io::Error> = file_exists_and_is_csv(file_path);
    if check.is_ok() {
        if !check.unwrap() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "File is not a .csv file",
            ));
        }
        // Open the file
        let file: File = File::open(file_path)?;

        let parsed_csv: HashMap<String, Vec<ParsedValue>> = match parse_csv_data(&file) {
            Ok(parsed_csv) => parsed_csv,
            Err(e) => {
                eprintln!("Failed to parse the csv data: {}", e);
                return Err(e);
            },
        };

        return Ok(parsed_csv);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    }
}

fn file_exists_and_is_csv(file_path: &str) -> Result<bool, io::Error> {
    let metadata: std::fs::Metadata = std::fs::metadata(file_path)?;
    Ok(metadata.is_file() && file_path.ends_with(".csv"))
}

#[derive(Debug, Clone)]
pub enum ParsedValue {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    String(String),
}

pub fn parse_csv_data(content: &File) -> Result<HashMap<String, Vec<ParsedValue>>, io::Error> {
    let reader = BufReader::new(content);
    let mut first_line = String::new();
    let mut parsed_csv_data: HashMap<String, Vec<ParsedValue>> = HashMap::new();
    let mut delimiter = ',';
    let mut column_names: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if first_line.is_empty() {
                    first_line = line.clone();
                    delimiter = match determine_delimiter(&first_line) {
                        Ok(delimiter) => delimiter,
                        Err(e) => {
                            eprintln!("Failed to determine delimiter");
                            return Err(e);
                        }
                    };
                    column_names = first_line.split(delimiter)
                        .map(|part| part.to_string())
                        .collect();
                } else {
                    let column_values: Vec<String> = split_csv_line(&line, delimiter).iter()
                        .map(|part| part.to_string())
                        .collect();
                    for (idx, value) in column_values.iter().enumerate() {
                        let parsed_value = find_data_type(value);
                        // Insert the parsed value into the HashMap
                        parsed_csv_data.entry(column_names[idx].clone())
                            .and_modify(|vec| vec.push(parsed_value.clone()))
                            .or_insert_with(|| vec![parsed_value.clone()]);
                    }
                }
            },
            Err(e) => {
                return Err(e); // Return error if any
            },
        }
    }

    Ok(parsed_csv_data)
}

// Dummy implementations of find_data_type and determine_delimiter for example purposes.
fn find_data_type(value: &str) -> ParsedValue {
    if let Ok(int_value) = value.parse::<i32>() {
        ParsedValue::Integer(int_value)
    } else if let Ok(float_value) = value.parse::<f64>() {
        ParsedValue::Float(float_value)
    } else if let Ok(bool_value) = value.parse::<bool>() {
        ParsedValue::Boolean(bool_value)
    } else {
        ParsedValue::String(value.to_string())
    }
}

fn split_csv_line(line: &str, delimiter: char) -> Vec<String> {
    let mut fields: Vec<_> = Vec::new();
    let mut current_field: String = String::new();
    let mut in_quotes = false;

    let chars: Vec<char> = line.chars().collect();
    let mut i: usize = 0;

    while i < chars.len() {
        let c: char = chars[i];

        if c == '"' {
            // Toggle the in_quotes flag
            in_quotes = !in_quotes;
        } else if c == delimiter && !in_quotes {
            // If not inside quotes, the comma marks the end of a field
            fields.push(current_field.trim().to_string());
            current_field.clear();
        }  else {
            // Otherwise, add the character to the current field
            current_field.push(c);
        }
        i += 1;
    }
    // Add the last field
    fields.push(current_field.trim().to_string());

    fields
}

fn determine_delimiter(first_line: &String) -> Result<char, io::Error>{

    // remove extraneos whitespace
    // let first_line: &str = first_line.trim();
    let possible_delimiter: [char; 5] = [',', ';', '\t', '|', ':'];
    //println!("First Line {}", first_line);
    let mut counts: HashMap<_, _> = std::collections::HashMap::new();
    for delimiter in &possible_delimiter {
        let count: usize = first_line.matches(*delimiter).count();
        counts.insert(delimiter.clone(), count);
        //println!("Count {} Delimiter {}", count, delimiter);
    }

    // Find delimiter with maximum count
    let (&detected_delimiter, _) = counts.iter().max_by_key(|&(_, count)| count).unwrap_or((&&',', &0));

    return Ok(detected_delimiter);
}


#[cfg(test)]
mod test {
    use super::*;

    const FILE_PATH_CSV: &str = "..\\..\\Datasets\\k-means\\abalone.csv";
    const FILE_PATH_TXT: &str = ".\\test.txt";

    #[test]
    fn read_csv_success() {
        let result = read_csv(FILE_PATH_CSV);
        assert!(result.is_ok());
        let contents = result.unwrap();
        assert!(!contents.is_empty());
    }

    #[test]
    fn read_csv_invalid_file_type() {
        let result = read_csv(FILE_PATH_TXT);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn read_csv_file_does_not_exist() {
        let result = read_csv("non_existent_file.csv");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn file_exist_check_test_exists_and_is_csv() {
        let result: Result<bool, io::Error> = file_exists_and_is_csv(FILE_PATH_CSV);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn file_exist_check_test_exists_but_no_csv() {
        let result: Result<bool, io::Error> = file_exists_and_is_csv(FILE_PATH_TXT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_file_does_not_exist() {
        let file_name: &str = "non_existent_file.csv";

        let result: Result<bool, io::Error> = file_exists_and_is_csv(file_name);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn delimiter_determination_is_working() {
        // Open the file
        let file = match File::open(FILE_PATH_CSV) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
                return;
            },
        };

        // Create a buffered reader
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();

        // Read the first line
        if let Err(e) = reader.read_line(&mut first_line) {
            eprintln!("Failed to read the first line: {}", e);
            return;
        }

        // Determine the delimiter
        let result = determine_delimiter(&first_line);
        match result {
            Ok(delimiter) => assert_eq!(delimiter, ','),
            Err(e) => eprintln!("Failed to determine delimiter: {}", e),
        }
    }

    #[test]
    fn test_parse_csv_data_columns_exist() {
        // Set the path to your CSV file here
        let file = File::open(FILE_PATH_CSV).expect("Failed to open the file");

        let parsed_result = parse_csv_data(&file).expect("Failed to parse CSV data");

        let expected_columns = vec![
            "Type".to_string(),
            "LongestShell".to_string(),
            "Diameter".to_string(),
            "Height".to_string(),
            "WholeWeight".to_string(),
            "ShuckedWeight".to_string(),
            "VisceraWeight".to_string(),
            "ShellWeight".to_string(),
            "Rings".to_string(),
        ];

        // Check that all expected columns are keys in the parsed HashMap
        for column in expected_columns.iter() {
            assert!(parsed_result.contains_key(column), "Column '{}' is missing in the parsed data", column);
        }
    }

    #[test]
    fn test_parse_csv_data_values_count() {
        // Set the path to your CSV file here
        let file = File::open(FILE_PATH_CSV).expect("Failed to open the file");

        let parsed_result = parse_csv_data(&file).expect("Failed to parse CSV data");

        // Expected total number of values in all columns
        let expected_total_values = 4177;

        // Calculate the total number of values in the parsed data
        let mut total_values = 0;
        for values in parsed_result.values() {
            total_values += values.len();
        }

        // Assert that the total number of values is as expected
        assert_eq!(
            total_values, expected_total_values * 9,
            "Total number of values does not match: expected {}, got {}",
            expected_total_values * 9, total_values
        );
    }

}
