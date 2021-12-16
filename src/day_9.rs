use std::collections::{HashSet, VecDeque};

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let (width, height) = (data[0].len() as i32, data.len() as i32);
    let grid: Vec<_> =
        data.iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap())).flatten().collect();

    let get_cell = |x, y| {
        if y < 0 || x < 0 || x > width - 1 || y > height - 1 {
            9
        } else {
            grid[(y * width + x) as usize]
        }
    };

    let mut basins = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let h = get_cell(x, y);
            if h < get_cell(x - 1, y)
                && h < get_cell(x + 1, y)
                && h < get_cell(x, y - 1)
                && h < get_cell(x, y + 1)
            {
                basins.push((x, y, h));
            }
        }
    }

    let mut part_2_basin_sizes: Vec<_> = basins
        .iter()
        .map(|(x, y, _)| {
            let mut size = 0;
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            let mut queue = VecDeque::from([(*x, *y)]);
            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();
                if visited.contains(&(x, y)) || get_cell(x, y) == 9 {
                    continue;
                }
                size += 1;
                visited.insert((x, y));
                queue.push_back((x - 1, y));
                queue.push_back((x + 1, y));
                queue.push_back((x, y - 1));
                queue.push_back((x, y + 1));
            }
            size
        })
        .collect();

    part_2_basin_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let part_1_answer: u32 = basins.iter().map(|(_, _, h)| h + 1).sum();
    let part_2_answer: i32 = part_2_basin_sizes.iter().take(3).product();

    (part_1_answer as i32, part_2_answer)
}

#[test]
fn test_sample() {
    let data: Vec<_> = r"2199943210
3987894921
9856789892
8767896789
9899965678"
        .split("\n")
        .collect();
    let (part_1, part_2) = main(data);
    assert_eq!(part_1, 15);
    assert_eq!(part_2, 1134);
}
