use std::fs::File;
use std::io::{BufRead, BufReader};

type Bond = (u8, u8, u8);

const BOARD_W: u8 = 5;
const BOARD_H: u8 = 5;

fn main() -> std::io::Result<()> {
    // let file = File::open("assets/test.txt")?;
    let file = File::open("assets/input.txt")?;

    let mut reader = BufReader::new(file);

    // Read numbers
    let numbers: Vec<u32> = {
        let mut numbers_text = String::new();
        reader
            .read_line(&mut numbers_text)
            .expect("Reading line of numbers");

        numbers_text
            .split(',')
            .filter_map(|it| it.trim().parse::<u32>().ok())
            .collect()
    };
    println!("Numbers: {:?}", numbers);

    // Fill boards and registry
    let mut registry: Registry = Registry::new();
    let mut boards: Vec<Box<Board>> = Vec::new();
    {
        let numbers = reader
            .lines()
            .filter(|it| !it.as_ref().unwrap().is_empty())
            .flat_map(|it| {
                it.unwrap()
                    .split(' ')
                    .filter_map(|x| x.trim().parse::<u8>().ok())
                    .collect::<Vec<_>>()
            });

        let mut acc: Vec<u8> = Vec::new();
        let mut col: u8 = 0;
        let mut row: u8 = 0;

        for num in numbers {
            acc.push(num);
            registry.register(num as u8, boards.len() as u8, col, row);

            col += 1;
            if col == BOARD_W {
                col = 0;
                row += 1;
            }
            if row == BOARD_H {
                row = 0;
            }

            if acc.len() as u8 == BOARD_W * BOARD_H {
                let board = Box::new(Board {
                    values: acc.clone(),
                    mark_mask: 0,
                });
                boards.push(board);

                acc.clear();
            }
        }
    };
    // boards.iter().for_each(|it| print_board(it));

    // Find winner
    let win_row_mask: u32 = {
        let mut mask: u32 = 0;
        for i in 0..BOARD_W {
            mask |= 1 << i;
        }
        mask
    };
    let win_col_mask: u32 = {
        let mut mask: u32 = 0;
        for i in 0..BOARD_H {
            mask |= 1 << i * BOARD_W;
        }
        mask
    };

    let mut win_num: Option<u8> = None;
    let mut win_board: Option<u8> = None;

    'nums: for num in numbers {
        let bonds = registry.get_bonds(num as u16);
        if bonds.is_none() {
            continue;
        }

        for (board_idx, col, row) in bonds.unwrap() {
            let board = &mut boards[board_idx as usize];
            let idx = row * BOARD_W + col;
            
            let mask: &mut u32 = &mut board.mark_mask;
            *mask = *mask | (1u32 << idx);

            board.values[idx as usize] = 0;

            let is_win = is_board_won(&board, col, row, win_col_mask, win_row_mask);
            if is_win > 0 {
                win_num = Some(num as u8);
                win_board = Some(board_idx);
                break 'nums;
            }
        }
    }

    if win_num.is_none() || win_board.is_none() {
        panic!("No winner")
    }

    // Winner found, calculate the result and print
    let winner = &boards[win_board.unwrap() as usize];
    println!("The winner is Board{} at number {}!", win_board.unwrap(), win_num.unwrap());
    print_board(winner);

    let ans = winner.values.iter().map(|it| *it as u32).sum::<u32>() * win_num.unwrap() as u32;
    println!("Answer: {:?}", ans);

    Ok(())
}

fn is_board_won(board: &Board, col: u8, row: u8, win_col_mask: u32, win_row_mask: u32) -> u8 {
    let col_mask = win_col_mask << col;
    // println!("\nboard mask: {:#034b}\nwin mask:   {:#034b}", board.mark_mask, col_mask);
    if board.mark_mask & col_mask == col_mask {
        return 1;
    }

    let row_mask = win_row_mask << (row * BOARD_W);
    if board.mark_mask & row_mask == row_mask {
        return 2;
    }

    0
}

fn print_board(board: &Board) {
    println!("Board {{");

    print!("  ");
    for i in 0..board.values.len() - 1 {
        print!("{}, ", board.values[i]);
        if (i + 1) % BOARD_W as usize == 0 {
            print!("\n  ")
        }
    }
    print!("{}\n", board.values[board.values.len() - 1]);
    println!("  mask: {:#034b}", board.mark_mask);
    println!("}}");
}

fn print_registry(registry: &mut Registry, numbers: Vec<u8>) {
    println!("Registry: {{");
    for n in &numbers {
        let bonds = registry.get_bonds(*n as u16);
        if bonds.is_none() {
            println!("  {}: none", n);
        } else {
            println!("  {}: {:?}", n, bonds.unwrap());
        }
    }
    println!("}}");
}

#[derive(Debug)]
struct Registry {
    values: Vec<Vec<u16>>,
}

impl Registry {
    fn new() -> Registry {
        let mut values: Vec<Vec<u16>> = Vec::with_capacity(100);
        (0..100).for_each(|_| {
            let bonds: Vec<u16> = Vec::new();
            values.push(bonds);
        });
        Registry { values: values }
    }

    fn register(&mut self, num: u8, board_idx: u8, col: u8, row: u8) {
        let num_max_bits: u8 = 7; // 0x00000111
        
        let mut bond: u16 = 0;
        bond = bond | ((board_idx as u16) << 6);
        bond = bond | (((col & num_max_bits) as u16) << 3);
        bond = bond | ((row & num_max_bits) as u16);

        // println!("Bond {} at board{}[{}, {}]: {:#034b}", num, board_idx, col, row, bond);
        let idx = num as usize;

        let bonds: &mut Vec<u16> = &mut self.values[idx];
        bonds.push(bond);
    }

    fn get_bonds(&mut self, num: u16) -> Option<Vec<Bond>> {
        let bond_data = &self.values[num as usize];
        if bond_data.len() == 0 {
            return None;
        }

        let num_max_bits: u16 = 7; // 0x00000111

        let bonds: Vec<Bond> = bond_data 
            .iter()
            .cloned()
            .map(|it| {
                let mut pack = it;

                let row = (pack & num_max_bits) as u8;
                pack = pack >> 3;

                let col = (pack & num_max_bits) as u8;
                pack = pack >> 3;

                let board_idx = pack as u8;

                (board_idx, col, row) as Bond
            })
            .collect();

        Some(bonds)
    }
}

#[derive(Debug)]
struct Board {
    values: Vec<u8>,
    mark_mask: u32,
}