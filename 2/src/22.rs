use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("assets/input.txt")?;
    let reader = BufReader::new(file);

    let coords = reader.lines()
        .map(|l| { 
            let cmd_str = l.unwrap();
            let cmd_vec: Vec<&str> = cmd_str.split(' ').collect();

            let cmd = cmd_vec[0].to_string();
            let value = cmd_vec[1].parse::<i32>().unwrap();

            (cmd, value)
        })
        .fold([0, 0, 0], |mut acc, (cmd, value)| {
            let aim = acc[2];
            match cmd.as_str() {
                "forward" => {
                    acc[0] += value;
                    acc[1] += value * aim;
                },
                "up" => acc[2] -= value,
                "down" => acc[2] += value,
                _ => panic!("Unknown command {}", cmd),
            };
            println!("{:?}", acc);
            acc
        });
    
    println!("Final coords: {:?}, answer: {}", coords, coords[0] * coords[1]);

    Ok(())
}