pub fn main(data: Vec<String>) {
    let values: Vec<u32> = data
        .into_iter()
        .map(|l| l.parse::<u32>().unwrap()) // Convert to u32
        .collect(); // ...into a vector

    let part_1_answer = part_1(&values[..]);
    let part_2_answer = part_2(&values[..]);

    println!("Part 1: {:?}", part_1_answer);
    println!("Part 2: {:?}", part_2_answer);
}

fn part_1(values: &[u32]) -> u32 {
    values
        .windows(2) // Sliding window over two values
        .filter(|values| values[1] > values[0]) // Retain values where value2 > value1
        .count() as u32 // Count how many they were
}

fn part_2(values: &[u32]) -> u32 {
    part_1(
        values
            .windows(3) // Sliding window over three values
            .map(|wnd| wnd.iter().sum()) // Sum the three values
            .collect::<Vec<u32>>() // Collect them into a vector
            .as_slice(), // ... and then to a slice
                         // Finally send this list of sums over to part_1
    )
}
