pub fn main(data: Vec<&str>) -> (u32, u32) {
    let bit_width = data[0].len();
    let bins: Vec<_> = data
        .into_iter()
        .map(|l| u32::from_str_radix(&l[..], 2).unwrap())
        .collect();

    let mut num_ones: Vec<u32> = Vec::new();
    num_ones.resize(bit_width, 0);
    let mut num_zeroes: Vec<u32> = Vec::new();
    num_zeroes.resize(bit_width, 0);

    for b in &bins {
        for n in 0..bit_width {
            let is_set = (b >> n) & 1 == 1;
            num_ones[n] += if is_set { 1 } else { 0 };
            num_zeroes[n] += if is_set { 0 } else { 1 };
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for n in 0..bit_width {
        if num_ones[n] > num_zeroes[n] {
            gamma = gamma | (1 << n);
        } else {
            epsilon = epsilon | (1 << n);
        }
    }

    fn trav<F>(search_vec: &Vec<u32>, f: F, current_bit: usize, bit_width: usize) -> u32 where F: Fn(usize, usize) -> bool {
        if search_vec.len() == 1 {
            return search_vec[0];
        }
        let (ones, zeroes): (Vec<u32>, Vec<u32>) = search_vec
            .into_iter()
            .partition(|f| ((**f >> (bit_width - 1 - current_bit)) & 1) == 1);
        if f(ones.len(), zeroes.len()) {
            trav(&ones, f, current_bit + 1, bit_width)
        }
        else {
            trav(&zeroes, f, current_bit + 1, bit_width)
        }
    }

    let oxygen_generator_rating = trav(&bins, |a, b| { a >= b }, 0, bit_width);
    let co2_scrubber_rating = trav(&bins,  |a, b| { a < b }, 0, bit_width);
    (
        epsilon * gamma,
        oxygen_generator_rating * co2_scrubber_rating,
    )
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
