use regex::Regex;

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let re = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
    let groups = re.captures(data[0]).unwrap();
    let x1 = groups.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let x2 = groups.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let y1 = groups.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let y2 = groups.get(4).unwrap().as_str().parse::<i32>().unwrap();

    let target = Rectangle { x1, x2, y1, y2 };

    let mut hits: Vec<(i32, i32, i32)> = Vec::new();
    for y in -1000..1000 {
        for x in 0..1000 {
            let (did_hit, maxy) = get_steps(x, y, &target);
            if did_hit {
                hits.push((x, y, maxy));
            }
        }
    }

    let &(_, _, max_y) =
        hits.iter().max_by(|(_, _, maxy1), (_, _, maxy2)| maxy1.cmp(maxy2)).unwrap();

    (max_y, hits.len() as i32)
}

struct Rectangle {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rectangle {
    fn is_point_in(&self, x: i32, y: i32) -> bool {
        x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
    }
}

fn get_steps(x_vel_start: i32, y_vel_start: i32, target_area: &Rectangle) -> (bool, i32) {
    let mut x_vel = x_vel_start;
    let mut y_vel = y_vel_start;

    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;

    loop {
        if target_area.is_point_in(x, y) {
            return (true, max_y);
        }
        if x > target_area.x2 || y < target_area.y1 {
            return (false, 0);
        }
        x += x_vel;
        y += y_vel;
        x_vel -= if x_vel != 0 { 1 } else { 0 };
        y_vel -= 1;
        if y > max_y {
            max_y = y;
        }
    }
}

#[test]
fn test_part1_samples_1_2_3_4() {
    let (part_1, part_2) = main(vec!["target area: x=20..30, y=-10..-5"]);
    assert_eq!(part_1, 45);
    assert_eq!(part_2, 112);
}
