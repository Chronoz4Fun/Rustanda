use rustanda::read_csv;


const FILE_PATH_CSV: &str = "C:\\Users\\Edward\\Desktop\\Datasets\\k-means\\abalone.csv";

fn main() {
    // Call the function you want to test
    match read_csv(FILE_PATH_CSV) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}

