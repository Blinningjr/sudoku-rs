use rand::prelude::SliceRandom;

use serde::{Deserialize, Serialize};

use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::Write;
use std::convert::TryInto;


#[derive(Debug, StructOpt)]
#[structopt(name = "suduko-tree", about = "A complete Suduko board generator.")]
struct Opt {

    /// Set num of boards
    #[structopt(short = "b", long = "boards", default_value = "1")]
    boards: usize,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let mut tree = Tree::init();

    println!("Generating Boards");
    let num_boards = opt.boards;
    let mut boards = vec!();

    let mut to_keep: u32 = 1;
    while 9_usize.pow(to_keep)< num_boards {
        to_keep += 1;
    }

    for _i in 0..num_boards {
        match tree.next() {
            Some(b) => boards.push(b),
            None => break,
        };
        tree.pop(to_keep.try_into().unwrap());
    }

    if opt.out_type == "file" {
        let path = opt.output.unwrap().join(opt.file_name.unwrap() + ".json");

        let mut file = match File::create(path){
            Ok(v) => v,
            Err(_) => {
                println!("Error: Could not open file");
                return;
            },
        };

        let json_string = match serde_json::to_string(&boards) {
            Ok(v) => v,
            Err(_) => {
                println!("Error: Faild to serialize value");
                return;
            }, 
        };

        match file.write_all(json_string.as_bytes()) {
            Ok(v) => v,
            Err(_) => {
                println!("Error: Faild to write to file");
                return;
            },
        };
    } else {
        for b in boards {
            println!("\n{}", b.print());
        }
    }

    println!("Done");
}


struct Tree {
    pub tree: Vec<Node>,
}


impl Tree {
    pub fn init() -> Self {
        Tree::new(Board::init())
    }


    pub fn new(board: Board) -> Self {
        Tree {
            tree: vec!(Node::new(board, Position::init())),
        } 
    }


    pub fn next(&mut self) -> Option<Board> {
        match self.tree.pop() {
            Some(mut node) => {
                let board = match node.next_board() {
                   Some(board)  => board,
                   None => return self.next(),
                };

                let new_position = match node.position.next() {
                    Some(pos) => pos,
                    None => {
                        self.tree.push(node);
                        return Some(board);
                    },
                };
                self.tree.push(node);
                self.tree.push(Node::new(board, new_position));
                self.next()
            },
            None => None,
        }
    }


    pub fn pop(&mut self, num: usize) {
        while self.tree.len() > num {
            self.tree.pop();
        }
    }
} 


struct Node {
    position: Position,
    board: Board,
    valid: Vec<i8>,
}


impl Node {
    pub fn new(board: Board, pos: Position) -> Node {
        let valid = board.get_valid(&pos);

        Node {
            position: pos,
            board: board,
            valid: valid,
        }
    }


    pub fn next_board(&mut self) -> Option<Board> {
        match self.valid.pop() {
            Some(num) => Some(self.board.new(&self.position, Some(num))),
            None => None,
        } 
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Board {
    board: [[Option<i8>; 9]; 9],
}


impl Board {
    pub fn init() -> Board {
        Board {
            board: [[None; 9]; 9],
        }
    }

    pub fn new(&self, pos: &Position, num: Option<i8>) -> Board {
        let mut new_board = self.clone();
        new_board.place(pos, num);
        new_board
    }

   
    pub fn place(&mut self, pos: &Position, num: Option<i8>) {
        self.board[pos.row as usize][pos.column as usize] = num;
    }


    pub fn print(&self) -> String {
        let mut result = "".to_string();
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0  {
                result = format!("{}- - - + - - - + - - -\n", result);
            } 
            for (i, x) in row.iter().enumerate() {
                if i % 3 == 0 && i != 0 {
                    result = format!("{}| ", result);
                }
                result = format!("{}{} ", result, match x { None => " ".to_string(), Some(v) => format!("{}", v),});
            } 
            result = format!("{}\n", result);
        }
        result
    }


    pub fn get_valid(&self, position: &Position) -> Vec<i8> {
        let mut valid = vec!();
        for i in 1..10 {
            if self.valid_placement(position, i) {
                valid.push(i);
            }
        }
        valid.shuffle(&mut rand::thread_rng());
        valid 
    }

    
    pub fn valid_placement(&self, position: &Position, num: i8) -> bool {
        self.valid_row(position, num) && self.valid_column(position, num) && self.valid_box(position, num)
    }


    pub fn valid_row(&self, position: &Position, num: i8) -> bool {
       for (i, column) in self.board[position.row as usize].iter().enumerate() {
            if i == position.column as usize {
                continue;
            }
            match column {
                Some(val) => {
                    if *val == num {
                        return false;
                    }
                },
                None => (),
            };
       }
       true
    }


    pub fn valid_column(&self, position: &Position, num: i8) -> bool {
        for (i, row) in self.board.iter().enumerate() {
            if i == (position.row as usize) {
                continue;
            }
            match row[position.column as usize] {
                Some(val) => {
                    if val == num {
                        return false;
                    }
                },
                None => (),
            };
        }
        true
    }


    pub fn valid_box(&self, position: &Position, num: i8) -> bool {
        let row_start = position.row - (position.row % 3);
        let column_start = position.column - (position.column % 3);
        
        for row_offset in 0..3 {
            for column_offset in 0..3 {
                if position.row == (row_start + row_offset)  && position.column == (column_start + column_offset) {
                    continue;
                }
                let row = self.board[(row_start + row_offset) as usize];
                let column = row[(column_start + column_offset) as usize];
                match column {
                    Some(val) => {
                        if val == num {
                            return false;
                        }
                    },
                    None => (),
                };
            }
        }
        true
    }
}


#[derive(Debug)]
struct Position {
    pub column: i8,
    pub row: i8,
}


impl Position {
    pub fn init() -> Position {
        Position {
            column: 0,
            row: 0,
        }
    }

    pub fn next(&self) -> Option<Position> {
        match (self.row, self.column) {
            (8, 8) => None,
            (row, 8) => Some(Position { row: row+1, column: 0, }),
            (row, column) => Some(Position { row: row, column: column+1, }),
        }
    }
}

