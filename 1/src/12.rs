use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("assets/input.txt")?;
    let reader = BufReader::new(file);

    let sliding_window_sums = reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .scan((-1, -1), |state, next| {
            let (a, b) = *state;
            *state = (b, next);

            Some((a, b, next))
        })
        .filter(|(a, b, _c)| *a > 0 && *b > 0)
        .map(|(a, b, c)| a + b + c)
        .map(|it| { println!("{}", it); it });

    let answer = to_chained_pairs(sliding_window_sums)
        .fold(0, |acc, [a, b]| if b > a { acc + 1 } else { acc }); 

    println!("Total: {}", answer);

    Ok(())
}

fn to_chained_pairs(numbers: impl Iterator<Item = i32>) -> impl Iterator<Item = [i32; 2]> {
    numbers
        .scan(-1, |state, next| {
            let prev = *state;
            *state = next;

            Some([prev, next])
        })
        .filter(|[a, _b]| *a > 0)
}