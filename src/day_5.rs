#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let lines: Vec<_> = data
        .iter()
        .map(|line| {
            let mut parts = line.split("->");
            let (x1, y1) = scan!(parts.next().unwrap().trim(), ",", i32, i32);
            let (x2, y2) = scan!(parts.next().unwrap().trim(), ",", i32, i32);
            Line { x1, y1, x2, y2}
        })
        .collect();

    let width = lines.iter().fold(0, |acc, cur| cur.x1.max(cur.x2).max(acc)) + 1;
    let height = lines.iter().fold(0, |acc, cur| cur.y1.max(cur.y2).max(acc)) + 1;
    
    let lap_lines = |board: &mut Vec<i32>, line: &Line, consider_diagonals: bool| {
        let dx = line.x2 - line.x1;
        let dy = line.y2 - line.y1;

        if !consider_diagonals && dx != 0 && dy != 0 {
            return;
        }

        let mut x = line.x1;
        let mut y = line.y1;
        let steps = if dx == 0 { dy.abs() } else { dx.abs() };
        for _ in 0..=steps {
            board[(y * width + x) as usize] += 1;
            x += dx.signum();
            y += dy.signum();
        }
    };

    let mut board_1 = vec![0; (width * height) as usize];
    let mut board_2 = vec![0; (width * height) as usize];
    for line in &lines {
        lap_lines(&mut board_1, line, false);
        lap_lines(&mut board_2, line, true);
    }

    let part_1_answer = board_1.iter().filter(|&&cell| cell >= 2).count() as i32;
    let part_2_answer = board_2.iter().filter(|&&cell| cell >= 2).count() as i32;

    (part_1_answer, part_2_answer)
}

#[test]
fn test_sample() {
    let data: Vec<_> = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
        .split("\n")
        .collect();

    let (part_1, part_2) = main(data);
    assert_eq!(part_1, 5);
    assert_eq!(part_2, 12);
}
