use std::fs;
fn main() {
    let contents = fs::read_to_string("../input.txt").unwrap();
    let lines: Vec<((i32, i32), (i32,i32))> = contents.split(|b| b == '\n').map(|line| {
        let ranges: Vec<&str> = line.split(",").collect();
        (get_range(ranges[0]), get_range(ranges[1]))
    }).collect();

    let fully_contained_sum: i32 = lines.iter().map(|pair| {
        if is_contained(pair.0, pair.1) || is_contained(pair.1, pair.0) {
             1
        } else {
            0
        }
    }).sum();
    
    println!("{}", fully_contained_sum);
}


fn get_range(input: &str) -> (i32, i32) {
    let range_ends: Vec<&str> = input.split("-").collect();
    (range_ends[0].parse::<i32>().unwrap(), range_ends[1].parse::<i32>().unwrap())
}

fn is_contained(left: (i32, i32), right: (i32, i32)) -> bool {
    left.0 >=right.0 && left.1 <= right.1
}