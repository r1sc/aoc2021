
pub fn main(data: Vec<&str>) -> (i32, i32) {
    let bit_width = data[0].len();
    let bins: Vec<_> = data
        .into_iter()
        .map(|l| u32::from_str_radix(&l[..], 2).unwrap())
        .collect();

    const BIT_WIDTH: usize = 12;

    let mut num_ones: Vec<u32> = Vec::new();
    num_ones.resize(BIT_WIDTH, 0);
    let mut num_zeroes: Vec<u32> = Vec::new();
    num_zeroes.resize(BIT_WIDTH, 0);

    for b in &bins {
        for n in 0..BIT_WIDTH {
            let is_set = (b >> n) & 1 == 1;
            num_ones[n] += if is_set { 1 } else { 0 };
            num_zeroes[n] += if is_set { 0 } else { 1 };
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for n in 0..BIT_WIDTH {
        if num_ones[n] > num_zeroes[n] {
            gamma = gamma | (1 << n);
        } else {
            epsilon = epsilon | (1 << n);
        }
    }

    fn trav(search_vec: &Vec<u32>, keep_larger: bool, current_bit: u32) -> u32 {
        if search_vec.len() == 1 {
            return search_vec[0];
        }
        let (ones, zeroes): (Vec<u32>, Vec<u32>) = search_vec
            .into_iter()
            .partition(|f| ((**f >> (((BIT_WIDTH - 1) as u32) - current_bit)) & 1) == 1);
        if keep_larger {
            if ones.len() >= zeroes.len() {
                trav(&ones, keep_larger, current_bit+1)        
            }
            else {
                trav(&zeroes, keep_larger, current_bit+1)
            }
        } else {
            if ones.len() < zeroes.len() {
                trav(&ones, keep_larger, current_bit+1)        
            }
            else {
                trav(&zeroes, keep_larger, current_bit+1)
            }
        }
    }

    let oxygen_generator_rating = trav(&bins, true, 0);
    let co2_scrubber_rating = trav(&bins, false, 0);

    ((epsilon * gamma) as i32, (oxygen_generator_rating * co2_scrubber_rating) as i32)
}

#[test]
fn test_sample() {
    let data = vec![
        "00100", 
        "11110", 
        "10110", 
        "10111", 
        "10101", 
        "01111", 
        "00111", 
        "11100", 
        "10000", 
        "11001",
        "00010", 
        "01010",
    ];
    let (power_consumption, oxy_co2) = main(data);
    assert!(power_consumption == 198);
    assert!(oxy_co2 == 230);
    println!("{} {}", power_consumption, oxy_co2)
}
