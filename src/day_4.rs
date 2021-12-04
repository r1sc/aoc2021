struct Cell {
    value: i32,
    marked: bool,
}
struct Board {
    cells: Vec<Cell>,
    already_won: bool,
}

impl Board {
    pub fn new(lines: &[&str]) -> Self {
        let cells: Vec<_> = lines
            .iter()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|col| Cell {
                        value: col.parse::<i32>().unwrap(),
                        marked: false,
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        Board {
            cells,
            already_won: false,
        }
    }

    pub fn mark(&mut self, drawn: i32) {
        for cell in &mut self.cells {
            if cell.value == drawn {
                cell.marked = true;
            }
        }
    }

    pub fn check_win(&self) -> bool {
        let col_win = (0..5)
            .map(|i| self.cells.iter().skip(i).step_by(5))
            .any(|mut col| col.all(|cell| cell.marked));

        let row_win = self.cells.chunks(5).any(|row| row.iter().all(|c| c.marked));

        col_win || row_win
    }

    pub fn get_score(&self) -> i32 {
        self.cells
            .iter()
            .filter(|&x| !x.marked)
            .map(|x| x.value)
            .sum()
    }
}

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let filtered_lines: Vec<_> = data
        .into_iter()
        .filter(|&l| l.is_empty() == false)
        .collect();

    let drawings: Vec<_> = filtered_lines[0]
        .split(",")
        .map(|d| d.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<_> = filtered_lines[1..].chunks(5).map(Board::new).collect();
    let mut winning_boards: Vec<(i32, i32)> = Vec::new();

    for drawn in drawings {
        for board in &mut boards {
            board.mark(drawn);
            if !board.already_won && board.check_win() {
                board.already_won = true;
                winning_boards.push((board.get_score(), drawn));
            }
        }
    }

    let ftw = winning_boards.first().expect("No first winner. Expected at least one to win?");
    let ltw = winning_boards.last().expect("No last winner. Expected at least one to win?");

    (ftw.0 * ftw.1, ltw.0 * ltw.1)
}

#[test]
fn test_sample() {
    let data: Vec<_> = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
"
    .split("\n")
    .collect();
    
    let (part_1, part_2) = main(data);
    assert!(part_1 == 4512);
    assert!(part_2 == 1924);
    println!("{}", part_1)
}
