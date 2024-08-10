use rustanda::{read_csv, parse_csv_data};
use std::fs::File;
use std::io::{self};

const FILE_PATH_CSV: &str = "C:\\Users\\Edward\\Desktop\\Datasets\\k-means\\abalone.csv";

fn main() -> io::Result<()> {
    // Ensure the path to your CSV file is correct
    let file = File::open(FILE_PATH_CSV)?;
    
    let csv_data = read_csv(FILE_PATH_CSV)?;

    /*if csv_data.is_empty() {
        println!("The file contains no lines.");
    } else {
        for line in csv_data {
            println!("{}", line);
        }
    }*/

    Ok(())
}
