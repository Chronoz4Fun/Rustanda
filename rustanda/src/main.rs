use rustanda::{read_csv, ParsedValue};
use std::fs::File;
use std::io::{self};
use std::collections::HashMap;

const FILE_PATH_CSV: &str = "C:\\Users\\Edward\\Desktop\\Datasets\\k-means\\accidents.csv";

fn print_parsed_csv_data(data: &HashMap<String, Vec<ParsedValue>>) {
    for (column_name, values) in data.iter() {
        println!("Column: {}", column_name);
        for value in values {
            match value {
                ParsedValue::Integer(i) => println!("    Integer: {}", i),
                ParsedValue::Float(f) => println!("    Float: {}", f),
                ParsedValue::Boolean(b) => println!("    Boolean: {}", b),
                ParsedValue::String(s) => println!("    String: {}", s),
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Ensure the path to your CSV file is correct
    //let file = File::open(FILE_PATH_CSV)?;
    
    let csv_data = read_csv(FILE_PATH_CSV)?;

    //print_parsed_csv_data(&csv_data);

    /*if csv_data.is_empty() {
        println!("The file contains no lines.");
    } else {
        for line in csv_data {
            println!("{}", line);
        }
    }*/

    Ok(())
}
