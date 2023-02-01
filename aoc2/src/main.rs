use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "../input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut my_score: i32 = 0;

    for line in buffered.lines() {
        let line = line.unwrap();
        let mut split_line = line.split_whitespace();
        let opponent_move = split_line.next().unwrap();
        let my_move = split_line.next().unwrap();
       //println!("Opponent Move: {opponent_move}, My Move: {my_move}");

        match opponent_move {
            "A" => match my_move {
                "X" => my_score += 4,
                "Y" => my_score += 8,
                "Z" => my_score += 3,
                _ => println!("Empty String"),
            },
            "B" => match my_move {
                "X" => my_score += 1,
                "Y" => my_score += 5,
                "Z" => my_score += 9,
                _ => println!("Empty String"),
            },
            "C" => match my_move {
                "X" => my_score += 7,
                "Y" => my_score += 2,
                "Z" => my_score += 6,
                _ => println!("Empty String"),
            },
            _ => println!("Empty String"),
        }

        println!("{my_score}");
    }

    Ok(())
}
