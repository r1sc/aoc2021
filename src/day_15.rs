use pathfinding::prelude::{absdiff, astar};

pub fn main(data: Vec<&str>) -> (u32, u32) {
    let height = data.len() as u32;
    let width = data[0].len() as u32;
    let grid: Vec<_> =
        data.iter().map(|row| row.chars().map(|c| c.to_digit(10).unwrap())).flatten().collect();

    type Point = (u32, u32);

    let wrap_around = |x: u32, y: u32| -> (u32, u32) { (x % width, y % width) };

    let get_risk = |x: u32, y: u32| -> u32 {
        let (wx, wy) = wrap_around(x, y);
        let tile_weight = (x / width) + (y / height);
        (((grid[(wy * width + wx) as usize] + tile_weight) - 1) % 9) + 1
    };

    let goal_1 = (width - 1, height - 1);
    let goal_2 = (width * 5 - 1, height * 5 - 1);

    let heuristic = |&(x, y): &Point, &(gx, gy): &Point| absdiff(x, gx) + absdiff(y, gy);

    let successors = |&(x, y): &Point, num_tiles: u32| {
        let pos_and_risk = |x: u32, y: u32| -> (Point, u32) { ((x, y), get_risk(x, y)) };

        let mut result = vec![];
        if x > 0 {
            result.push(pos_and_risk(x - 1, y));
        }
        if y > 0 {
            result.push(pos_and_risk(x, y - 1));
        }
        if x < (width * num_tiles) - 1 {
            result.push(pos_and_risk(x + 1, y));
        }
        if y < (height * num_tiles) - 1 {
            result.push(pos_and_risk(x, y + 1));
        }
        result
    };

    let result_part_1 =
        astar(&(0, 0), |p| successors(p, 1), |xy| heuristic(xy, &goal_1), |&p: &Point| p == goal_1)
            .unwrap();
    let result_part_2 =
        astar(&(0, 0), |p| successors(p, 5), |xy| heuristic(xy, &goal_2), |&p: &Point| p == goal_2)
            .unwrap();

    (result_part_1.1.into(), result_part_2.1.into())
}

#[test]
fn test_sample() {
    let data: Vec<_> = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
        .split("\n")
        .collect();
    let (part_1, part_2) = main(data);
    assert_eq!(part_1, 40);
    assert_eq!(part_2, 315);
}
