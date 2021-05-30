use crate::game::{InvalidMove, PlayerKind, Tiles};
use std::io::{self, Write};
use std::process;

pub fn prompt_move() -> (usize, usize, usize) {
    loop {
        print!("Enter move using the syntax 'row column size' (e.g. 1A1): ");

        io::stdout().flush().expect("Failed to flush stdout");

        let line = read_line();

        match parse_move(&line) {
            Ok((row, col, size)) => break (row, col, size),
            Err(InvalidMove(invalid_str)) => {
                eprintln!("Invalid move: '{}'. Please try again.", invalid_str,)
            }
        }
    }
}

pub fn parse_move(input: &str) -> Result<(usize, usize, usize), InvalidMove> {
    if input.len() != 3 {
        return Err(InvalidMove(input.to_string()));
    }

    let row = match &input[0..1] {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        _ => return Err(InvalidMove(input.to_string())),
    };

    let col = match &input[1..2] {
        "A" | "a" => 0,
        "B" | "b" => 1,
        "C" | "c" => 2,

        invalid => return Err(InvalidMove(invalid.to_string())),
    };

    let size = match input[2..3].parse::<usize>() {
        Ok(x) => x,
        _ => return Err(InvalidMove(input.to_string())),
    };

    Ok((row, col, size))
}

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input.is_empty() {
        println!();

        process::exit(0);
    }

    let len_without_newline = input.trim_end().len();
    input.truncate(len_without_newline);

    input
}

impl Tiles {
    pub fn print(self: &Self) {
        print!("  ");
        for j in 0..self[0].len() as u8 {
            print!(" {}", (b'A' + j) as char);
        }
        println!();

        for i in 0..3 {
            print!(" {}", i + 1);
            for j in 0..3 {
                let tile = self[i][j];
                print!(
                    " {}",
                    match tile {
                        Some((PlayerKind::X, _)) => "x",
                        Some((PlayerKind::O, _)) => "o",
                        _ => "_",
                    }
                );
            }

            print!("    ");

            for j in 0..3 {
                let tile = self[i][j];
                print!(
                    " {}",
                    match tile {
                        Some((_, x)) => x.to_string(),
                        _ => "_".to_string(),
                    }
                );
            }
            println!();
        }

        println!();
    }
}
