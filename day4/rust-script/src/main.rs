use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use std::fs;
use std::vec;

fn main() -> Result<()> {
    println!();

    let f = fs::read_to_string("../input.txt").expect("Failed to read input");
    println!("Given input:\n{f}");

    let mut i = f.lines();

    let first_line = i.next().context("No lines present")?;

    fn parse_input<'a>(line: impl Iterator<Item = &'a str>) -> Result<Vec<i32>> {
        line.map(|x| x.parse::<i32>().context("Could not parse line"))
            .collect()
    }

    let chosen = parse_input(first_line.split(",")).context("Could not parse first line")?;
    let mut boards = vec![];
    let mut starred = vec![];

    for line in i {
        if line.trim().is_empty() {
            boards.push(vec![]);
            starred.push(vec![]);
            continue;
        }
        let board = boards
            .last_mut()
            .expect("Expected newline before each board");
        let star = starred
            .last_mut()
            .expect("Expected newline before each board");
        board.push(parse_input(line.split_whitespace()).context("Could not parse board line")?);
        star.push(vec![0; board.last_mut().unwrap().len()]);
    }

    let mut bingo_scores = vec![];

    for choice in chosen {
        for (board, star) in boards.iter().zip(&mut starred) {
            for (row, star_row) in board.iter().zip(star.iter_mut()) {
                for (number, star_value) in row.iter().zip(star_row) {
                    if *number == choice {
                        *star_value = 1;
                    }
                }
            }

            // Check whether we've gotten bingo
            let mut any_success = false;
            let mut col_success = vec![true; star.len()];
            for star_row in &*star {
                if any_success {
                    break;
                }
                let mut row_success = true;
                for i in 0..star_row.len() {
                    let star_value = star_row[i];
                    if star_value != 1 {
                        row_success = false;
                        col_success[i] = false;
                    }
                }
                if row_success {
                    // This row is all 1's, so this board has a bingo.
                    any_success = true;
                    break;
                }
            }
            if any_success || col_success.iter().any(|x| *x) {
                // This board has a bingo (either row or column)
                // Calculate the score
                let mut score = 0;
                for (row, star_row) in board.iter().zip(star) {
                    for (number, star_value) in row.iter().zip(star_row) {
                        if *star_value == 0 {
                            score += number;
                        }
                    }
                }
                bingo_scores.push(choice * score);
            }
        }
        if !bingo_scores.is_empty() {
            // If at least one board has won bingo on this round, stop immediately
            break;
        }
    }
    let max_score = bingo_scores
        .iter()
        .max()
        .context("No board has reached a bingo")?;

    println!("{max_score}");
    fs::write("../output.txt", format!("{max_score}\n")).context("Could not write to output")?;

    Ok(())
}
