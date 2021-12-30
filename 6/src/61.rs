use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // let file = File::open("assets/test.txt")?;
    let file = File::open("assets/input.txt")?;

    let reader = BufReader::new(file);

    let mut fish: Vec<i8> = {
        let line = reader.lines().next().unwrap().unwrap();
        line.split(",")
            .map(|it| it.parse::<i8>().unwrap())
            .collect()
    };

    for day in 0..80 {
        for i in 0..fish.len()  {
            if fish[i] == 0 {
                fish[i] = 7;
                fish.push(8);
            }
            fish[i] -= 1;
        }
        println!("After day {}: {:?}", day + 1, fish.len());
        // println!("After day {}: {:?}", day + 1, fish);
    }

    println!("Answer: {:?}", fish.len());

    Ok(())
}
