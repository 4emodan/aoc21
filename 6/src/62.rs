use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // let file = File::open("assets/test.txt")?;
    let file = File::open("assets/input.txt")?;

    let reader = BufReader::new(file);

    let fish: Vec<u8> = {
        let line = reader.lines().next().unwrap().unwrap();
        line.split(",")
            .map(|it| it.parse::<u8>().unwrap())
            .collect()
    };
    println!("Fish: {:?}", fish);

    let mut fish_by_day: [u64; 9] = [0; 9];
    for day in fish {
        fish_by_day[day as usize] += 1;
    }
    println!("Fish by day before: {:?}", fish_by_day);

    for day in 0..256 {
        let new_fish = fish_by_day[0];
        fish_by_day.rotate_left(1);
        fish_by_day[6] += new_fish;
        // println!("After day {}: {:?}", day, fish_by_day);
    }
    println!("Fish by day after: {:?}", fish_by_day);

    let ans = fish_by_day.iter().fold(0, |acc, it| acc + it);
    println!("Answer: {:?}", ans);

    Ok(())
}
