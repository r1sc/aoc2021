pub fn main(data: Vec<String>) {
    let (a, b, c) = data.into_iter().fold((0, 0, 0), |(a, b, c), line| {
        let mut parts = line.split(" ");
        let instruction = parts.by_ref().next().unwrap();
        let value = parts.by_ref().next().unwrap().parse::<i32>().unwrap();

        match &instruction[..] {
            "forward" => (a + value, b, c + b * value),
            "up" => (a, b - value, c),
            "down" => (a, b + value, c),
            _ => panic!("Unknown instruction"),
        }
    });

    println!("Part 1: {}, part 2: {}", a * b, a * c)
}
