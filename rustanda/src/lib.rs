// src/lib.rs

use std::fs::File;
use std::io::{self, Read};

pub fn read_csv(file_path: &str) -> io::Result<String> {
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
        let mut file = File::open(file_path)?;

        // Create a String to hold the file contents
        let mut contents = String::new();

        // Read the file into the String
        file.read_to_string(&mut contents)?;

        Ok(contents)
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

#[cfg(test)]
mod test {
    use super::*;

    const FILE_PATH_CSV: &str = "C:\\Users\\Edward\\Desktop\\Datasets\\k-means\\abalone.csv";
    const FILE_PATH_TXT: &str = "C:\\Users\\Edward\\Desktop\\Rustanda\\rustanda\\test.txt";

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
}
