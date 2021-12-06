pub fn main(data: Vec<&str>) -> (u64, u64) {
    let state: Vec<_> = data[0]
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let calc_population = |days: usize| {
        let mut timers = vec![0; 9];
        for i in 0..state.len() {
            let timer = state[i];
            timers[timer as usize] += 1;
        }

        let mut population = state.len() as u64;
        for _ in 0..days {
            let amount_new_fishes = timers[0];
            population += amount_new_fishes;
            for i in 0..timers.len() - 1 {
                timers[i] = timers[i + 1];
            }
            timers[6] += amount_new_fishes;
            timers[8] = amount_new_fishes;
        }
        population
    };

    (calc_population(80), calc_population(256))
}

#[test]
fn test_sample() {
    let data: Vec<_> = vec!["3,4,3,1,2"];
    let (part_1, part_2) = main(data);
    assert_eq!(part_1, 5934);
    assert_eq!(part_2, 26984457539);
}
