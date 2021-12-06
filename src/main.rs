use std::fs::File;
use std::io::{Write, BufReader, BufRead};

fn main()
{
    // Prompt the user for an input file
    let prompt_str = "Please enter the input file path: ";
    println!("{}", prompt_str);
    std::io::stdout().flush().unwrap();

    // Read the input file path from the user
    let mut file_path_str = String::new();
    std::io::stdin().read_line(&mut &mut file_path_str).expect("Failed to read input file path");

    // Open and read the file contents
    // This should work, but doesn't!
    let file = File::open( file_path_str).expect("Failed to open file");
    // Hardcoding works!
    //let file = File::open( "input.txt").expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    // Print the file contents
    for line in buf_reader.lines()
    {
        println!("{}", line.unwrap());
    }
}
