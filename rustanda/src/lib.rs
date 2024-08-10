// src/lib.rs

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use std::any::Any;

pub fn read_csv(file_path: &str) -> Result<Vec<String>, io::Error> {
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

        let parsed_csv: Vec<String> = match parse_csv_data(&file) {
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

pub fn parse_csv_data(content: &File) -> Result<Vec<String>, io::Error> {
    let reader = BufReader::new(content);
    let mut lines = Vec::new();
    let mut first_line: String = String::new();
    let mut parsed_csv_data: HashMap<String, Box<dyn Any>> = std::collections::HashMap::new();
    let mut delimiter: char = ',';
    let mut column_name: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if first_line.is_empty() {
                    first_line = line.clone();
                    delimiter = match determine_delimiter(&first_line) {
                        Ok(delimiter)   => delimiter,
                        Err(e) => {
                             eprint!("Failed to determine delimiter");
                             return Err(e);
                        },

                    };
                    column_name = first_line.split(delimiter).map(|part: &str| part.to_string()).collect();
                } else {
                    let column_values: Vec<String> = line.split(delimiter).map(|part: &str| part.to_string()).collect();
                    for (idx, value) in column_values.iter().enumerate() {
                        parsed_csv_data.entry(column_name[idx].clone());
                    }
                    lines.push(line); // Add line to the vector
                }
            },
            Err(e) => {
                return Err(e); // Return error if any
            },
        }
    }

    //println!("Total lines collected: {}", lines.len()); // Debugging output to check the count
    Ok(lines)
}

    /*loop {
        first_line.clear(); // Clear the buffer before the next read
        let bytes_read = reader.read_line(&mut first_line)?;

        // Check if we reached the end of the file
        if bytes_read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "EoF reached without seeing a line with content",
            ));
        }

        // Break the loop if a non-empty line is found
        if !first_line.trim().is_empty() {
            break;
        }
    }


    let delimiter = match determine_delimiter(first_line.clone()) {
        Ok(detected_delimiter) =>  detected_delimiter,
        Err(e) => { eprintln!("Failed to determine delimiter: {}", e); return Err(e);},
    };

    let column_name: Vec<String> = first_line.split(delimiter).map(|part: &str| part.to_string()).collect();


    return Ok(column_name);*/

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
}
