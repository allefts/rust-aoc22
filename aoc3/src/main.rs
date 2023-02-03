fn main() {
    //Part 1
    // println!(
    //     "{}",
    //     include_bytes!("../input.txt")
    //         .split(|b| *b == b'\n')
    //         .map(|l| l.split_at(l.len() / 2))
    //         .map(|(a, b)| b
    //             .iter()
    //             .filter(|b| a.contains(b))
    //             .map(|b| if *b >= b'a' {
    //                 (b - b'a') as i16 + 1
    //             } else {
    //                 (b - b'A') as i16 + 27
    //             })
    //             .next()
    //             .unwrap())
    //         .sum::<i16>(),
    // );
    

    //Part 2
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|set| set[0]
                .iter()
                .find(|b| set[1].contains(b) && set[2].contains(b))
                .unwrap())
            .map(|b| if *b >= b'a' {
                (b - b'a') as i16 + 1
            } else {
                (b - b'A') as i16 + 27
            })
            .sum::<i16>(),
    );
}
