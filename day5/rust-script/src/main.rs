use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use itertools::Itertools;
use std::fs;

struct Vent {
    start_point: (i32, i32),
    end_point: (i32, i32),
}

impl Iterator for Vent {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_point {
        } else {
            None
        }
    }
}

fn main() -> Result<()> {
    println!();

    let debug = false;

    let f = fs::read_to_string("../input.txt").expect("Failed to read input");
    if debug {
        println!("Given input:\n{f}");
    }

    let mut vents = vec![];
    for line in f.lines() {
        let mut vent = vec![];
        for point in line.split("->") {
            let (start, end) = point
                .trim()
                .split(",")
                .map(|coord| coord.parse::<i32>().context("Could not parse coordinate"))
                .collect_tuple()
                .context("Could not parse point")?;
            vent.push((start?, end?));
        }
        // Ensure the first point in the vent is the smaller of the two given endpoints
        vent.sort();
        // Add to vents
        vents.push(vent);
    }

    // Sort the vents
    // vents.sort();

    // After sorting
    // if debug {
    //     for vent in &vents {
    //         println!(
    //             "{:?}->{:?}",
    //             vent.first().expect("Could not get the start of a vent"),
    //             vent.last().expect("Could not get the end of a vent")
    //         );
    //     }
    // }

    fn unwrap_vent(vent: &Vec<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
        let start_point = vent.first().expect("Could not get the start of a vent");
        let end_point = vent.last().expect("Could not get the end of a vent");
        (*start_point, *end_point)
    }

    let mut seen = vec![];

    for i in 0..vents.len() {
        let vent = &vents[i];
        let (start_point, end_point) = unwrap_vent(vent);
        for j in (i + 1)..vents.len() {
            let other_vent = &vents[j];
            let (other_start_point, other_end_point) = unwrap_vent(other_vent);

            if start_point <= other_start_point && other_start_point <= end_point {
                for point in other_start_point..other_end_point {}
            } else if other_start_point <= start_point && start_point <= other_end_point {
            }
        }
    }

    /*
    for i in 0..vents.len() {
        let vent = &vents[i];
        let (start_point, end_point) = unwrap_vent(vent);

        for j in (i + 1)..vents.len() {
            let other_vent = &vents[j];
            let (other_start_point, other_end_point) = unwrap_vent(other_vent);

            let vent_is_vertical = start_point.0 == end_point.0;
            let vent_is_horizontal = start_point.1 == end_point.1;
            // let vent_is_diagonal = !vent_is_vertical && !vent_is_horizontal;

            let other_vent_is_vertical = other_start_point.0 == other_end_point.0;
            let other_vent_is_horizontal = other_start_point.1 == other_end_point.1;
            // let other_vent_is_diagonal = !other_vent_is_vertical && !other_vent_is_horizontal;

            // Check the different overlapping cases
            if vent_is_vertical && other_vent_is_vertical {
                // Check if these two are in the same column
                if start_point.0 != other_start_point.0 {
                    // These two vents are in different columns, abort
                    continue;
                }
                // Not needed but...
                assert_eq!(
                    start_point.0, other_start_point.0,
                    "Vents {:#?}->{:#?} and {:#?}->{:#?} were not in the same column",
                    start_point, end_point, other_start_point, other_end_point
                );

                // Otherwise, these two vents are in the same column
                // Check what points (if any) these two vents have that overlap

                // Check if vent is above other_vent
                if start_point.1 > other_end_point.1 {
                    // No overlap
                    continue;
                }

                // Check if vent is below other_vent
                if end_point.1 < other_start_point.1 {
                    // No overlap
                    continue;
                }

                // Must have some overlap
                let start = max(start_point, other_start_point);
                let end = min(end_point, other_end_point);
                for k in start.1..(end.1 + 1) {
                    if seen.contains(&(start.0, k)) {
                        continue;
                    }
                    seen.push((start.0, k));
                }
            } else if vent_is_horizontal && other_vent_is_horizontal {
                // Check if these two are in the same row
                if start_point.1 != other_start_point.1 {
                    // These two vents are in different rows, abort
                    continue;
                }
                // Otherwise, these two vents are in the same row
                // Check what points (if any) these two vents have that overlap

                // Check if vent is to the right of other_vent
                if start_point.0 > other_end_point.0 {
                    // No overlap
                    continue;
                }

                // Check if vent is to the left of other_vent
                if end_point.0 < other_start_point.0 {
                    // No overlap
                    continue;
                }

                // Must have some overlap
                let start = max(start_point, other_start_point);
                let end = min(end_point, other_end_point);
                for k in start.0..(end.0 + 1) {
                    if seen.contains(&(k, start.1)) {
                        continue;
                    }
                    seen.push((k, start.1));
                }
            } else {
                // One of the vents is vertical, and the other is horizontal
                // At most, only one point could overlap between the two vents
                if vent_is_vertical && other_vent_is_horizontal {
                    // Check if vent is in the horizontal region
                    if other_start_point.0 <= start_point.0 && start_point.0 <= other_end_point.0 {
                        // This could look like the following:
                        //  |    |
                        // -|-  ---

                        // Check if other vent is in vertical region
                        if start_point.1 <= other_start_point.1
                            && other_start_point.1 <= end_point.1
                        {
                            // Only 1 point can overlap
                            if !seen.contains(&(start_point.0, other_start_point.1)) {
                                seen.push((start_point.0, other_start_point.1));
                            }
                        }
                    }
                } else if vent_is_horizontal && other_vent_is_vertical {
                    // Check if other vent is in the horizontal region
                    if start_point.0 <= other_start_point.0 && other_start_point.0 <= end_point.0 {
                        // This could look like the following:
                        //  |    |
                        // -|-  ---

                        // Check if vent is in vertical region
                        if other_start_point.1 <= start_point.1
                            && start_point.1 <= other_end_point.1
                        {
                            // Only 1 point can overlap
                            if !seen.contains(&(other_start_point.0, start_point.1)) {
                                seen.push((other_start_point.0, start_point.1));
                            }
                        }
                    }
                }
            }
        }
    }
    */

    let points = seen.len();
    println!("Length of seen: {points}");

    if debug {
        println!("Seen contains:");
        for item in seen {
            println!("{:?}", item);
        }
    }

    fs::write("../output.txt", format!("{points}\n")).context("Could not write to output")?;

    Ok(())
}
