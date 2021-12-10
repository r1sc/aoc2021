use std::collections::{HashMap, VecDeque};

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let facit = HashMap::from([
        (')', ('(', 3_u64)),
        (']', ('[', 57_u64)),
        ('}', ('{', 1197_u64)),
        ('>', ('<', 25137_u64)),
    ]);

    let get_line_points = |line: &&str| {
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in line.chars() {
            match facit.get(&c) {                
                Some((left, score)) => {
                    if stack.pop_front() != Some(*left) {
                        return (true, *score);
                    }
                }
                None => stack.push_front(c),
            };
        }
        let autocorrect_score = stack.drain(..).fold(0, |acc, c| {
            (acc * 5) + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    l => panic!("Didnt expect {} on the stack!", l),
                }
        });
        (false, autocorrect_score)
    };

    let (broken, mut incomplete): (Vec<(bool, u64)>, Vec<(bool, u64)>) = data
        .iter()
        .map(get_line_points)
        .partition(|(is_broken, _)| *is_broken);

    incomplete.sort();

    let part_1_answer: u64 = broken.iter().map(|(_, score)| score).sum();
    let part_2_answer: u64 = incomplete[incomplete.len() / 2].1;

    (part_1_answer as i32, part_2_answer as i32)
}

#[test]
fn test_sample() {
    let data: Vec<_> = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        .split("\n")
        .collect();

    let (part_1, part_2) = main(data);
    assert_eq!(part_1, 26397);
    assert_eq!(part_2, 288957);
}
