pub fn main(data: Vec<&str>) -> (i32, i32) {
    let values: Vec<i32> = data
        .into_iter()
        .map(|l| l.parse::<i32>().unwrap()) // Convert to u32
        .collect(); // ...into a vector

    let part_1_answer = part_1(&values[..]);
    let part_2_answer = part_2(&values[..]);

    (part_1_answer, part_2_answer)
}

fn part_1(values: &[i32]) -> i32 {
    values
        .windows(2) // Sliding window over two values
        .filter(|values| values[1] > values[0]) // Retain values where value2 > value1
        .count() as i32 // Count how many they were
}

fn part_2(values: &[i32]) -> i32 {
    part_1(
        values
            .windows(3) // Sliding window over three values
            .map(|wnd| wnd.iter().sum()) // Sum the three values
            .collect::<Vec<i32>>() // Collect them into a vector
            .as_slice(), // ... and then to a slice
                         // Finally send this list of sums over to part_1
    )
}

#[test]
fn test_sample() {
    let data: Vec<_> = r"199
200
208
210
200
207
240
269
260
263"
    .split("\n")
    .collect();
    let (part_1, part_2) = main(data);
    assert_eq!(part_1, 7);
    assert_eq!(part_2, 5);
}
