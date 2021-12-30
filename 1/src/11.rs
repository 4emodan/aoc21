use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("assets/input.txt")?;
    let reader = BufReader::new(file);

    let answer = reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .scan(-1, |state, next| {
            let prev = *state;
            *state = next;

            Some((prev, next))
        })
        .filter(|(a, _b)| *a > 0)
        .fold(0, |acc, (a, b)| {
            if b > a {
                println!("{} > {}", b, a);
                acc + 1
            } else {
                acc
            }
        });

    println!("Total: {}", answer);

    Ok(())
}
