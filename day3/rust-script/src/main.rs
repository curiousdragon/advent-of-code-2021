// imports
// for file paths
use std::path::Path;
// for files
use std::fs::File;
// for io
use std::io::prelude::*;
// for vectors
use std::vec::Vec;
// for chars
use std::str::Chars;

fn diagnostics(input: Vec<String>) -> i32 {
    // Calculate the gamma rate and the epsilon rate.
    // 
    // Input: vector of strings (binary numbers)
    // Output: vector with 2 integer elements
    //
    // Example:
    // ```
    // data = ["00100", "11110", "10110", "10111", "10101", "01111",
    //      "00111", "11100", "10000", "11001", "00010", "01010"]
    // diagnostics(data)
    // ```
    //
    // Expected:
    // ```
    // [22, 9]
    // ```

    // Initialize the counts
    let mut counts = vec![];
    let length: usize = input[0].len();
    for _ in 0..length {
        counts.push(vec![0, 0]);
    }
    // Go through each input
    for line in input {
        let mut chars: Chars = line.chars();
        for i in 0..length {
            let bin = chars.next().unwrap();
            if bin.to_string() == "0" {
                counts[i][0] += 1;
            } else {
                counts[i][1] += 1;
            }
        }
    }
    // Initialize the rates: [gamma, epsilon]
    let mut vec = vec!["".to_string(), "".to_string()];
    // Go through the counts
    for i in 0..length {
        let inner_count = &counts[i];
        if inner_count[0] > inner_count[1] {
            vec[0] = format!("{}{}", vec[0], "0").to_string();
            vec[1] = format!("{}{}", vec[1], "1").to_string();
        } else {
            vec[0] = format!("{}{}", vec[0], "1").to_string();
            vec[1] = format!("{}{}", vec[1], "0").to_string();
        }
    }
    (isize::from_str_radix(&vec[0], 2).unwrap() * 
        isize::from_str_radix(&vec[1], 2).unwrap()).try_into().unwrap()
}

fn main() {
    // Create a path to the input file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open path in read-only mode
    let mut f = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(f) => f,
    };

    // Read file contents
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("Failed to read {}: {}", display, why),
        Ok(_) => println!("Successfully read {}", display),
    };

    // Create a vector for the input
    let input: Vec<String> = s.split_whitespace()
        .map(|s| s.to_string())
        .collect();
    println!("{}", input[0]);
    

    // Call the function
    let output_data : i32 = diagnostics(input);
    let output : String = output_data.to_string();

    // Create a path to the output file
    let path = Path::new("output.txt");
    let display = path.display();

    // Open path in write-only mode
    let mut f2 = match File::create(&path) {
        Err(why) => panic!("Failed to create {}: {}", display, why),
        Ok(f2) => f2,
    };

    // Write output to file
    match f2.write_all(output.as_bytes()) {
        Err(why) => panic!("Failed to write {}, {}", display, why),
        Ok(_) => println!("Successfully wrote {} to {}", output, display),
    };

    // Write ending newline to file
    match f2.write_all("\n".as_bytes()) {
        Err(why) => panic!("Failed to write {}, {}", display, why),
        Ok(_) => println!("Successfully wrote {} to {}", "a newline", display),
    };
}
