use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("assets/input.txt")?;
    let reader = BufReader::new(file);

    let mut sums = reader.lines()
        .map(|l| l.unwrap().chars().collect())
        .fold(Vec::new(), |mut acc: Vec<u32>, values: Vec<char>| {
            if acc.len() < values.len() {
                acc.resize(values.len() + 1, 0)
            }
            for i in 0..values.len() {
                let value = values[i].to_digit(10).unwrap();
                acc[i] += value; 
            }
            *acc.last_mut().unwrap() += 1;

            acc
        });
    
    let positions_count = sums.len() - 1;
    let count = *sums.last().unwrap();
    let half_count = (count / 2, count % 2);

    let mut gamma_rate: u32 = 0;

    for i in 0..positions_count {
        let diff = sums[i] as i32 - half_count.0 as i32;

        sums[i] = if diff > 0 {
            1
        } else if diff < 0 || half_count.1 > 0 {
            0
        } else {
            panic!("Equal quantity of zeroes and ones!")
        };

        gamma_rate = gamma_rate | sums[i];
        gamma_rate = gamma_rate << 1;
    }
    gamma_rate = gamma_rate >> 1;

    let epsilon_rate = {
        let trailing_positions = 32 - positions_count; 
        ((!gamma_rate) << trailing_positions) >> trailing_positions
    };

    println!("Gamma  : {:#034b}\nEpsilon: {:#034b}", gamma_rate, epsilon_rate);
    println!("Total {}, values: {:?}, gamma: {}, epsilon: {}", count, sums, gamma_rate, epsilon_rate);
    println!("Answer: {}", gamma_rate * epsilon_rate);

    Ok(())
}