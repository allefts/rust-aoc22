use std::fs;
use std::vec::Vec;

//struct Elf {
//    num_calories: i32
//}

fn main() {
    println!("Advent of Code 2022, Problem 1, Part 1");

    let elf_contents = fs::read_to_string("../input.txt").expect("Should be able to read file");
    let split_elf_contents: Vec<&str> = elf_contents.split("\n\n").collect();
    let mut end_elf_cals: Vec<i32> = Vec::new();

    for elf in split_elf_contents.iter() {
        let elf_cals: Vec<&str> = elf.split('\n').collect();
        let mut temp_cal: i32 = 0;

        for cals in elf_cals.iter() {
            let my_cals: i32 = cals.parse().unwrap_or(0); 
            temp_cal += my_cals;
        }
        end_elf_cals.push(temp_cal);
    }

    //let max_cals = end_elf_cals.iter().max().unwrap();
    //let max_cals_idx = end_elf_cals.iter().position(|&r| r == *max_cals).unwrap() - 1; 

    println!("Advent of Code 2022, Problem 1, Part 2");

    end_elf_cals.sort();
    let top_three_elf_cals: i32 = end_elf_cals.iter().rev().take(3).sum();
    println!("{top_three_elf_cals}");
}

