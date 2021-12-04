use std::cell::Cell;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point {
    is_checked: Cell<bool>,
    val: u32
}

struct PlayBoard {
    board: Vec<Vec<Point>>,
}

impl PlayBoard {
    fn flag_number(&mut self, number: &u32) {
        for row in &self.board {
            for point in row {
                if point.val == *number {
                    point.is_checked.set(true);
                }
            }
        }
    }
    fn winner(&self) -> bool {
        let mut index = 0;
        for row in &self.board {
            let bools = row.iter().map(|point | point.is_checked.get()).collect::<Vec<bool>>();
            let sum_of_row = bools.iter().map(|x| *x as u32).sum::<u32>();
            if sum_of_row == 5 {
                return true;
            }

            // now check the column at index i, since i is a same size matrix
            let column_is_checked= &self.board
                .iter()  // iterate over rows
                .map(|x| x[index].is_checked.get()) // get the icolumn-th element from each row
                .collect::<Vec<bool>>();

            let sum_of_col = column_is_checked.iter().map(|x| *x as u32).sum::<u32>();
            if sum_of_col == 5 {
                return true;
            }

            index += 1;
        }

        return false;
    }

    fn score(&mut self) -> u32 {
        let values = &self.board.iter_mut().flat_map(|row| row.iter().filter(|p| p.is_checked.get() == false).map(|p| p.val).collect::<Vec<u32>>()).collect::<Vec<u32>>();
        return values.iter().sum::<u32>();
    }
}

pub fn star_one() -> u32 {
    // 1. Read solution as a single line
    // 2. Create playboard
    let lines_of_file: Vec<String> = get_lines("src/day_four/input.txt");
    let playboard_lines = &lines_of_file[2..].to_vec();
    let solution: &Vec<u32> = &lines_of_file[0].split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut boards: Vec<PlayBoard> = vec![];

    for chunk in playboard_lines.chunks(6) {
        boards.push(parse_playboard_from_chunk(chunk));
    }

    for (idx, number) in solution.iter().enumerate() {
        for (index, board) in boards.iter_mut().enumerate() {
            board.flag_number(number);

            if board.winner() {
                let score = board.score();
                println!("Number {}  --- Board {} won at idx {} with score {}!", number, index, idx, score);
                return score * number;
            }
        }
    }

    return 0;
}

fn parse_playboard_from_chunk(chunk: &[String]) -> PlayBoard {
    let mut board: Vec<Vec<Point>> = vec![];

    for row in chunk {
        if row.len() > 0 {
            let columns: Vec<Point> = row.split_whitespace()
                                        .map(|x| Point {
                                            is_checked: Cell::from(false),
                                            val: x.trim().parse::<u32>()
                                            .unwrap()}).collect();
            board.push(columns);
        }
    }

    let mut pb = PlayBoard { board };

    return pb;
}

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.expect("Could not parse line")).collect();
}
