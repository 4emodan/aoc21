use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // let file = File::open("assets/test.txt")?;
    let file = File::open("assets/input.txt")?;

    let reader = BufReader::new(file);

    let mut bits = reader.lines().map(|l| {
        l.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .fold(Vec::new(), |mut acc: Vec<u8>, digit: u8| {
                acc.push(digit);
                acc
            })
    }).peekable();

    let mut root = Node {
        value: 42,
        children: [None, None],
        followers: 0,
    };

    let size = bits.by_ref().peek().unwrap().len();
    println!("Size determined: {}", size);

    { // Populate tree
        for v in bits {
            v.len();
            let mut current_node = &mut root;
            for d in v {
                let idx = d as usize;
                current_node.followers += 1;

                // Create node if missing
                if current_node.children[idx].is_none() {
                    let node = Node {
                        value: d,
                        followers: 0,
                        children: [None, None],
                    };
                    current_node.children[idx] = Some(Box::new(node));
                }
                current_node = current_node.children[idx].as_mut().unwrap().as_mut();
            }
        }
    }
    println!("Tree populated, {} values loaded", root.followers);
    // println!("Tree:\n{:?}", root);

    let oxygen_answer: u32 = {
        let bits = find_rating(&root, size, true, true);
        let value = to_u32(&bits);
        println!("Oxygen generator rating: {}\n\t{:?}\n\t{:#034b}", value, bits, value);
        value
    };
    let co2_answer: u32 = {
        let bits = find_rating(&root, size, false, false); 
        let value = to_u32(&bits);
        println!("CO2 scrubber rating: {}\n\t{:?}\n\t{:#034b}", value, bits, value);
        value
    };

    println!("Final answer: {}", oxygen_answer * co2_answer);

    Ok(())
}

fn find_rating(root: &Node, size: usize, is_most_common: bool, choose_one_if_equal: bool) -> Vec<u8> {
    let mut answer: Vec<u8> = Vec::new();
    
    let mut current_node = root;
    for i in 0..size  {
        let node0 = &current_node.children[0];
        let node1 = &current_node.children[1];

        let total_count = current_node.followers;
        let half_count: (i32, u32) = (total_count as i32 / 2i32, total_count % 2);

        let next_node = match (node0, node1) {
            (None, None) => panic!("Can't all branches be empty at {} level!", i),
            (None, Some(b)) => b.as_ref(),
            (Some(b), None) => b.as_ref(),
            (Some(b0), Some(b1)) => {
                let diff = b0.followers as i32 - half_count.0;

                if diff > 0 { // zeroes more common
                    if is_most_common { b0 } else { b1 }
                } else if diff < 0 || half_count.1 > 0 { // ones more common
                    if is_most_common { b1 } else { b0 }
                } else { // equal
                    if choose_one_if_equal { b1 } else { b0 }
                }.as_ref()
            },
        };
        answer.push(next_node.value);
        current_node = next_node;
    }

    answer
}

fn to_u32(bits: &Vec<u8>) -> u32 {
    let mut out: u32 = 0;
    for b in bits {
        out = out | (*b as u32);
        out = out << 1;
    }
    out >> 1
}

#[derive(Debug)]
struct Node {
    value: u8,
    children: [Option<Box<Node>>; 2],
    followers: u32,
}
