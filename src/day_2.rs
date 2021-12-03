use crate::scan;

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let (a, b, c) = data.into_iter().fold((0, 0, 0), |(a, b, c), line| {
        let (instruction, value) = scan!(line, " ", String, i32);

        match &instruction[..] {
            "forward" => (a + value, b, c + b * value),
            "up" => (a, b - value, c),
            "down" => (a, b + value, c),
            _ => panic!("Unknown instruction"),
        }
    });

    (a * b, a * c)
}

#[test]
fn test_sample() {
    let data = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];
    let (part1_answer, part2_answer) = main(data);
    assert!(part1_answer == 150);
    assert!(part2_answer == 900);
}
