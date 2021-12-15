struct State {
    width: i32,
    height: i32,
    flashes: i32,
    old_flashes: i32,
    state: Vec<u32>,
}

impl State {
    fn new(width: i32, height: i32, state: Vec<u32>) -> Self {
        State {
            width,
            height,
            flashes: 0,
            old_flashes: 0,
            state,
        }
    }

    fn increase(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        } else {
            self.state[(y * self.width + x) as usize] += 1;
            if self.state[(y * self.width + x) as usize] == 10 {
                self.flash(x, y);
            }
        }
    }

    fn flash(&mut self, x: i32, y: i32) {
        self.flashes += 1;
        self.increase(x - 1, y);
        self.increase(x + 1, y);
        self.increase(x, y - 1);
        self.increase(x, y + 1);
        self.increase(x - 1, y - 1);
        self.increase(x + 1, y - 1);
        self.increase(x - 1, y + 1);
        self.increase(x + 1, y + 1);
    }

    fn tick(&mut self) {
        self.old_flashes = self.flashes;

        for y in 0..self.height {
            for x in 0..self.width {
                self.increase(x, y);
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if self.state[(y * self.width + x) as usize] > 9 {
                    self.state[(y * self.width + x) as usize] = 0;
                }
            }
        }
    }

    fn print_grid(&self) {
        let rows = self.state.iter().as_slice().chunks(self.width as usize);
        for row in rows {
            println!("{:?}", row);
        }
    }
}

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let width = data[0].len() as i32;
    let height = data.len() as i32;
    let grid: Vec<_> = data
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()))
        .flatten()
        .collect();

    let mut state = State::new(width, height, grid);

    let mut all_flashed_at = 0;
    let mut flashes_after_100_steps = 0;
    for i in 0..1000 {
        state.tick();
        if i < 100 {
            flashes_after_100_steps = state.flashes;
        }
        if state.flashes - state.old_flashes == 100 {
            all_flashed_at = i + 1;
            break;
        }
    }

    state.print_grid();
    (flashes_after_100_steps, all_flashed_at)
}

#[test]
fn test_sample() {
    let data: Vec<_> = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
        .split("\n")
        .collect();

    let (part_1, part_2) = main(data);
    assert_eq!(part_1, 1656);
    assert_eq!(part_2, 195);
}
