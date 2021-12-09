use std::collections::HashMap;

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let parts: Vec<_> = data
        .iter()
        .map(|line| {
            let parts: Vec<_> = line.split("|").collect();
            let patterns: Vec<_> = parts[0].trim().split(" ").collect();
            let output_values: Vec<_> = parts[1].trim().split(" ").collect();
            (patterns, output_values)
        })
        .collect();

    let part_1: usize = parts
        .iter()
        .map(|(_, output_values)| {
            output_values
                .iter()
                .filter(|v| v.len() == 2 || v.len() == 4 || v.len() == 3 || v.len() == 7).count()
        })
        .sum();

    let part_2 = parts
        .iter()
        .map(|(patterns, output_values)| {
            let digits = decode_line(patterns, output_values);
            digits.iter().fold(0, |acc: i32, cur| (acc << 3) + (acc << 1) + (*cur as i32))
        })
        .sum();
    (part_1 as i32, part_2)
}

struct Seg(Vec<char>);

impl Seg {
    pub fn from_str(a: &str) -> Self {
        Seg(a.chars().collect())
    }

    pub fn sub(&self, b: &Self) -> Self {
        let v: Vec<char> = self
            .0
            .iter()
            .filter(|&seg| !b.0.iter().any(|&f| f == *seg))
            .map(|f| *f)
            .collect();
        Self(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_char(&self) -> char {
        *self.0.first().unwrap()
    }
}

// 1. diff 7 (3 segs) med 1 (2 segs) - ger oss seg 0
fn find_seg_0(seven: &Seg, one: &Seg) -> Seg {
    seven.sub(one)
}

// 2. 2, 3, 5 har 5 segment -
// ta bort de som är samma som 1:an - den som har 3 segment kvar är 3:an
fn find_seg_0_3_6(five_segs: &Vec<Seg>, one: &Seg) -> Option<Seg> {
    for seg in five_segs {
        let diff = seg.sub(one);
        if diff.len() == 3 {
            return Some(diff);
        }
    }
    None
}

// för de 3 segmenten, ta bort seg 0, kvar är 3 och 6. diffa detta med 4 (4 segs)
// - detta ger oss seg 3, den som är kvar är seg 6
fn find_seg_3_6(five_segs: &Vec<Seg>, one: &Seg, seg_0: &Seg, four: &Seg) -> (Seg, Seg) {
    let three_seg = find_seg_0_3_6(five_segs, one).unwrap();

    let seg_3_6 = three_seg.sub(seg_0);
    let seg_6 = seg_3_6.sub(four);
    let seg_3 = seg_3_6.sub(&seg_6);
    (seg_3, seg_6)
}

fn find_seg_1(four: &Seg, one: &Seg, seg_3: &Seg) -> Seg {
    four.sub(one).sub(seg_3)
}

fn find_seg_4(eight: &Seg, four: &Seg, seg_0: &Seg, seg_3: &Seg, seg_6: &Seg) -> Seg {
    eight.sub(four).sub(seg_0).sub(seg_3).sub(seg_6)
}

fn find_seg_5(zero_six_nine: Vec<Seg>, all_known_segs: Vec<&Seg>) -> Option<Seg> {
    for a in zero_six_nine {
        let mut removed = a;
        for seg in all_known_segs.iter() {
            removed = removed.sub(seg);
        }
        if removed.len() == 1 {
            return Some(removed);
        }
    }
    None
}

fn find_seg_2(eight: Seg, all_known_segs: Vec<&Seg>) -> Seg {
    let mut removed = eight;
    for seg in all_known_segs.iter() {
        removed = removed.sub(seg);
    }
    removed
}

fn decode(encoded: &str, seg_map: &HashMap<char, u8>, digit_map: &HashMap<u8, u8>) -> u8 {
    let mut key: u8 = 0;
    for c in encoded.chars() {
        let bit_pos = *seg_map.get(&c).unwrap();
        key |= 1 << bit_pos;
    }

    *digit_map.get(&key).unwrap()
}

fn decode_line(patterns: &Vec<&str>, output_values: &Vec<&str>) -> Vec<u8> {
    let one = Seg::from_str(patterns.iter().find(|&p| p.len() == 2).unwrap());
    let four = Seg::from_str(patterns.iter().find(|&p| p.len() == 4).unwrap());
    let seven = Seg::from_str(patterns.iter().find(|&p| p.len() == 3).unwrap());
    let eight = Seg::from_str(patterns.iter().find(|&p| p.len() == 7).unwrap());

    let seg_0 = find_seg_0(&seven, &one);
    let five_segs: Vec<_> = patterns
        .iter()
        .filter(|p| p.len() == 5)
        .map(|&s| Seg::from_str(s))
        .collect();

    let (seg_3, seg_6) = find_seg_3_6(&five_segs, &one, &seg_0, &four);
    let seg_1 = find_seg_1(&four, &one, &seg_3);
    let seg_4 = find_seg_4(&eight, &four, &seg_0, &seg_3, &seg_6);

    let six_segs: Vec<_> = patterns
        .iter()
        .filter(|p| p.len() == 6)
        .map(|&s| Seg::from_str(s))
        .collect();

    let seg_5 = find_seg_5(six_segs, vec![&seg_0, &seg_1, &seg_3, &seg_4, &seg_6]).unwrap();
    let seg_2 = find_seg_2(eight, vec![&seg_0, &seg_1, &seg_3, &seg_4, &seg_5, &seg_6]);

    let all_segments = HashMap::from([
        (seg_0.to_char(), 0),
        (seg_1.to_char(), 1),
        (seg_2.to_char(), 2),
        (seg_3.to_char(), 3),
        (seg_4.to_char(), 4),
        (seg_5.to_char(), 5),
        (seg_6.to_char(), 6),
    ]);

    let mapping = HashMap::from([
        (0x77, 0),
        (0x24, 1),
        (0x5D, 2),
        (0x6D, 3),
        (0x2E, 4),
        (0x6B, 5),
        (0x7B, 6),
        (0x25, 7),
        (0x7F, 8),
        (0x6F, 9),
    ]);

    output_values
        .iter()
        .map(|v| decode(v, &all_segments, &mapping))
        .collect()
}

/*
------- TESTS ----------
*/

#[test]
fn test_find_seg_0() {
    let seg_0 = find_seg_0(&Seg::from_str("dab"), &Seg::from_str("ab"));
    assert_eq!(seg_0.len(), 1);
    assert_eq!(seg_0.to_char(), 'd');
}

#[test]
fn test_find_seg_3_6() {
    let seg_0 = find_seg_0(&Seg::from_str("dab"), &Seg::from_str("ab"));
    let five_segs: Vec<_> = vec!["cdfbe", "gcdfa", "fbcad"]
        .iter()
        .map(|&s| Seg::from_str(s))
        .collect();
    let (seg_3, seg_6) = find_seg_3_6(
        &five_segs,
        &Seg::from_str("ab"),
        &seg_0,
        &Seg::from_str("eafb"),
    );

    assert_eq!(seg_3.to_char(), 'f');
    assert_eq!(seg_6.to_char(), 'c');
}

#[test]
fn test_find_seg_1() {
    let seg_0 = find_seg_0(&Seg::from_str("dab"), &Seg::from_str("ab"));
    let five_segs: Vec<_> = vec!["cdfbe", "gcdfa", "fbcad"]
        .iter()
        .map(|&s| Seg::from_str(s))
        .collect();
    let (seg_3, _) = find_seg_3_6(
        &five_segs,
        &Seg::from_str("ab"),
        &seg_0,
        &Seg::from_str("eafb"),
    );

    let seg_1 = find_seg_1(&Seg::from_str("eafb"), &Seg::from_str("ab"), &seg_3);
    assert_eq!(seg_1.len(), 1);
    assert_eq!(seg_1.to_char(), 'e');
}

#[test]
fn test_find_seg_4() {
    let seg_0 = find_seg_0(&Seg::from_str("dab"), &Seg::from_str("ab"));
    let five_segs: Vec<_> = vec!["cdfbe", "gcdfa", "fbcad"]
        .iter()
        .map(|&s| Seg::from_str(s))
        .collect();
    let (seg_3, seg_6) = find_seg_3_6(
        &five_segs,
        &Seg::from_str("ab"),
        &seg_0,
        &Seg::from_str("eafb"),
    );

    let seg_4 = find_seg_4(
        &Seg::from_str("acedgfb"),
        &Seg::from_str("eafb"),
        &seg_0,
        &seg_3,
        &seg_6,
    );
    assert_eq!(seg_4.len(), 1);
    assert_eq!(seg_4.to_char(), 'g');
}

#[test]
fn test_find_seg_5() {
    let seg_0 = find_seg_0(&Seg::from_str("dab"), &Seg::from_str("ab"));
    let five_segs: Vec<_> = vec!["cdfbe", "gcdfa", "fbcad"]
        .iter()
        .map(|&s| Seg::from_str(s))
        .collect();
    let (seg_3, seg_6) = find_seg_3_6(
        &five_segs,
        &Seg::from_str("ab"),
        &seg_0,
        &Seg::from_str("eafb"),
    );
    let seg_1 = find_seg_1(&Seg::from_str("eafb"), &Seg::from_str("ab"), &seg_3);
    let seg_4 = find_seg_4(
        &Seg::from_str("acedgfb"),
        &Seg::from_str("eafb"),
        &seg_0,
        &seg_3,
        &seg_6,
    );

    let zero_six_nine: Vec<_> = vec!["cefabd", "cdfgeb", "cagedb"]
        .iter()
        .map(|&s| Seg::from_str(s))
        .collect();
    let seg_5 = find_seg_5(zero_six_nine, vec![&seg_0, &seg_1, &seg_3, &seg_4, &seg_6]);
    let eight = Seg::from_str("acedgfb");
    let seg_2 = find_seg_2(
        eight,
        vec![&seg_0, &seg_1, &seg_3, &seg_4, &seg_5.unwrap(), &seg_6],
    );

    assert_eq!(seg_2.to_char(), 'a');
}

#[test]
fn test_decode() {
    let results = decode_line(
        &vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ],
        &vec!["cdfeb", "fcadb", "cdfeb", "cdbaf"],
    );

    assert_eq!(results.len(), 4);
    let mut result_iter = results.iter();
    assert_eq!(*result_iter.next().unwrap(), 5);
    assert_eq!(*result_iter.next().unwrap(), 3);
    assert_eq!(*result_iter.next().unwrap(), 5);
    assert_eq!(*result_iter.next().unwrap(), 3);
}

#[test]
fn test_part_1() {
    let data: Vec<_> =
        r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            .split("\n")
            .collect();
    let (part_1, _) = main(data);
    assert_eq!(part_1, 26);
}

#[test]
fn test_part_2() {    
    let data: Vec<_> =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
            .split("\n")
            .collect();
    let (_, part_2) = main(data);
    assert_eq!(part_2, 5353);

}
