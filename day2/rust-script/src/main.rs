// imports
// for file paths
use std::path::Path;
// for files
use std::fs::File;
// for io
use std::io::prelude::*;
// for vectors
// use std::vec::Vec;
// for lines
use std::str::Lines;

fn calculate_coords(lines: Lines<>) -> i32 {
    // Calculate the horizontal position and the depth
    // according to the directions given in DATA.
    // Return a list whose first element is the horizontal position,
    // and whose second element is the depth.
    // 
    // Input: vector of vectors of integers
    // Output: vector with 2 integer elements
    //
    // Example:
    // ```
    // data = [["forward", 5], ["down", 5], ["forward", 8],
    //      ["up", 3], ["down", 8], ["forward", 2]]
    // calculate_coords(data)
    // ```
    // Expected:
    // ```
    // [15, 10]
    // ```

    // Initialize the coordinates:
    // [horizontal position, depth, aim]
    let mut vec = vec![0, 0, 0];
    // Go through each line of directions
    for line in lines {
        // Split the line into direction and value
        let mut inner_vec_str = line.split_whitespace();
        let direction: String = inner_vec_str.next().unwrap().to_string();
        let value: i32 = inner_vec_str.next().unwrap().
            to_string().parse().unwrap();

        // Different behavior depending on direction
        if direction == "forward" {
            vec[0] += value;
            vec[1] += vec[2] * value;
        } else if direction == "down" {
            vec[2] += value;
        } else if direction == "up" {
            vec[2] -= value;
        } else {
            // This direction does not exist
            println!("Error: invalid direction: {}", direction);
        }
    }
    vec[0] * vec[1]
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
    // Iterator the lines of the input
    let lines = s.lines();

    // Call the function - part 1
    let output_data : i32 = calculate_coords(lines);
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
