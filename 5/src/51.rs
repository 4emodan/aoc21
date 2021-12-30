use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (u16, u16);
type Line = (Point, Point);

struct Map {
    w: usize,
    h: usize,
    values: Vec<u16>,
}

fn main() -> std::io::Result<()> {
    // let file = File::open("assets/test.txt")?;
    let file = File::open("assets/input.txt")?;

    let reader = BufReader::new(file);

    let mut w: usize = 0;
    let mut h: usize = 0;

    // Read lines
    let lines: Vec<Line> = reader
        .lines()
        .map(|line| {
            let line_text = line.unwrap();
            let mut line_parts = line_text.split(" -> ");
            let from = parse_point(line_parts.next().unwrap());
            let to = parse_point(line_parts.next().unwrap());

            w = cmp::max(w, cmp::max(from.0 as usize, to.0 as usize));
            h = cmp::max(h, cmp::max(from.1 as usize, to.1 as usize));

            (from, to) as Line
        })
        .collect();

    println!("Creating map {}, {}", w + 1, h + 1);
    let mut map: Map = Map::new(w + 1, h + 1);

    let hv_lines: Vec<Line> = lines
        .iter()
        .filter(|line| {
            let p1 = line.0;
            let p2 = line.1;
            p1.0 == p2.0 || p1.1 == p2.1
        })
        .copied()
        .collect::<Vec<Line>>();

    for line in hv_lines {
        map.mark(&line);
    }

    let ans = map.values
        .iter()
        .fold(0, |acc, x| acc + if *x > 1 { 1 } else { 0 });

    println!("Lines: {:?}", lines);
    // map.print();

    println!("Answer: {:?}", ans);

    Ok(())
}

fn parse_point(text: &str) -> Point {
    let mut point_parts = text.split(",");
    let x = point_parts.next().unwrap().parse::<u16>().unwrap();
    let y = point_parts.next().unwrap().parse::<u16>().unwrap();
    (x, y)
}

impl Map {
    fn new(w: usize, h: usize) -> Map {
        Map {
            w: w,
            h: h,
            values: vec![0; w * h],
        }
    }

    fn mark(&mut self, line: &Line) {
        // println!("Marking line {:?}", line);
        let min_x: u16 = cmp::min(line.0 .0, line.1 .0);
        let max_x: u16 = cmp::max(line.0 .0, line.1 .0);
        let min_y: u16 = cmp::min(line.0 .1, line.1 .1);
        let max_y: u16 = cmp::max(line.0 .1, line.1 .1);

        for i in min_x..(max_x + 1) {
            for j in min_y..(max_y + 1) {
                // println!("  Marking {}, {}", i, j);
                self.values[(self.w * (j as usize) + (i as usize))] += 1;
            }
        }
    }

    fn print(&self) {
        print!("Map {{");
        for j in 0..self.h {
            print!("\n");
            for i in 0..self.w {
                let val = self.values[(self.w * j + i) as usize];
                if val == 0 {
                    print!(" . ");
                } else {
                    print!(" {} ", val);
                }
            }
        }
        println!("\n}}");
    }
}
