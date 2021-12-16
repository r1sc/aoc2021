pub fn main(data: Vec<&str>) -> (i32, i32) {
    let state: Vec<_> = data[0].split(",").map(|i| i.parse::<i32>().unwrap()).collect();

    let max_horizontal_position = *state.iter().max().unwrap();

    let calc_fuel = |x: i32| -> (i32, i32) {
        let distances: Vec<_> = state.iter().map(|cur| (cur - x).abs()).collect();
        let fuel_cost_v1 = distances.iter().sum();
        let fuel_cost_v2 = distances.iter().map(|&distance| (1..=distance).sum::<i32>()).sum();

        (fuel_cost_v1, fuel_cost_v2)
    };

    let mut cheapest_fuel_v1 = i32::MAX;
    let mut cheapest_fuel_v2 = i32::MAX;

    for x in 0..=max_horizontal_position {
        let (fuel_required_v1, fuel_required_v2) = calc_fuel(x);
        if fuel_required_v1 < cheapest_fuel_v1 {
            cheapest_fuel_v1 = fuel_required_v1;
        }
        if fuel_required_v2 < cheapest_fuel_v2 {
            cheapest_fuel_v2 = fuel_required_v2;
        }
    }

    (cheapest_fuel_v1, cheapest_fuel_v2)
}

#[test]
fn test_sample() {
    let data: Vec<_> = vec!["16,1,2,0,4,2,7,1,2,14"];
    let (part_1, part_2) = main(data);
    assert_eq!(part_1, 37);
    assert_eq!(part_2, 168);
}
