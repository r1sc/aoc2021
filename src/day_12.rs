use std::collections::{HashMap, HashSet};

pub fn main(data: Vec<&str>) -> (i32, i32) {
    let tuples: Vec<_> = data.iter().map(|line| {
        let mut splitted = line.split("-");
        (splitted.next().unwrap(), splitted.next().unwrap())
    }).collect();

    let mut edges = HashMap::new();
    for (k, v) in tuples {
        edges.entry(k).or_insert_with(Vec::new).push(v)
    }

    // let traverse = |banned: &mut HashSet<&str>, visited_small_once: bool, currentPath: &mut Vec<&str>, so_far: &Vec<Vec<&str>>| -> Vec<Vec<&str>> {
    //     let current = currentPath.last().unwrap();
    //     let mut visited_small_once = visited_small_once;
    //     if banned.contains(current) {
    //         if *current == "start" || visited_small_once {
    //             return Vec::new();
    //         }
    //         visited_small_once = true;
    //     }

    //     if *current == "end" {
    //         let mut new_vec = so_far.to_vec();
    //         new_vec.push(currentPath.to_vec());
    //         return new_vec;
    //     }

    //     let c = current.chars().next().unwrap();
    //     if c.is_lowercase() {
    //         banned.insert(current);
    //     }

    //     Vec::new()
    // };

    todo!()
}

// #[test]
// fn test_sample() {
//     let data: Vec<_> = r"start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end"
//         .split("\n")
//         .collect();

//     let (part_1, part_2) = main(data);
//     assert_eq!(part_1, 1656);
//     assert_eq!(part_2, 195);
// }
