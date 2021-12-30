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
        .map(|(cmd, value)| -> [i32; 2] {
            match cmd.as_str() {
                "forward" => [value, 0],
                "up" => [0, -value],
                "down" => [0, value],
                _ => panic!("Unknown command {}", cmd),
            }
        })
        .fold([0, 0], |acc, [x, y]| [acc[0] + x, acc[1] + y]);
    
    println!("Final coords: {:?}, answer: {}", coords, coords[0] * coords[1]);

    Ok(())
}