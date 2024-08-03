// src/lib.rs

use std::fs::File;
use std::io::{self, Read};

pub fn read_csv(file_path:&str) -> io::Result<String> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Create a String to hold the file contents
    let mut contents = String::new();

    // Read the file into the String
    file.read_to_string(&mut contents)?;

    // Print the contents
    println!("{}", contents);

    Ok(contents)
}

fn file_exists_and_is_csv(file_path:&str) -> Result<bool, io::Error> {
    let metadata = std::fs::metadata(file_path)?;
    Ok(metadata.is_file() && file_path.ends_with(".csv"))
}


#[cfg(test)]
mod test {
    use super::*;

    const FILE_PATH_CSV:&str = "C:\\Users\\Edward\\Desktop\\Datasets\\k-means\\abalone.csv";
    const FILE_PATH_TXT:&str = "C:\\Users\\Edward\\Desktop\\Rustanda\\rustanda\\test.txt";

    #[test]
    fn basic_test(){
        match read_csv(FILE_PATH_CSV) {
            Ok(contents) => {
                // Perform some assertions or checks on the contents
                assert!(!contents.is_empty(), "File contents should not be empty");
            }
            Err(e) => panic!("Failed to read the file: {:?}", e),
        }
    }

    #[test]
    fn file_exist_check_test_exists_and_is_csv() {
        let result = file_exists_and_is_csv(FILE_PATH_CSV);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn file_exist_check_test_exists_but_no_csv() {
        let result = file_exists_and_is_csv(FILE_PATH_TXT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_file_does_not_exist() {
        let file_name = "non_existent_file.csv";

        let result = file_exists_and_is_csv(file_name);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), io::ErrorKind::NotFound);
    }




}
