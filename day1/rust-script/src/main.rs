// imports
// for file paths
use std::path::Path;
// for files
use std::fs::File;
// for io
use std::io::prelude::*;
// for vectors
use std::vec::Vec;

fn count_increase(vec: Vec<i32>) -> i32 {
    // Return the number of times an element of DATA increases from its
    // previous element.
    //
    // Input: slice of integers (i32)
    // Output: integer i32)
    //
    // Example:
    // ```
    // data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    // count_increase(data) // expected: 7
    // ```

    let mut count = 0;
    for i in 1..vec.len() {
        if vec[i] > vec[i-1] {
            count += 1;
        }
    }
    count
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
    let vec: Vec<i32> = s.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", vec[0]);

    // Call the function
    let output_data : i32 = count_increase(vec);
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
