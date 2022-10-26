use core::slice::Iter;
use std::fmt;
use std::slice::Chunks;

struct Sudoku {
    board: [u8; 81],
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku { board: [0; 81] }
    }

    fn serialize(&self) -> String {
        self.board
            .iter()
            .map(|cell| match *cell {
                0 => '.',
                _ => (*cell + '0' as u8) as char,
            })
            .collect()
    }

    fn deserialize(s: &str) -> Sudoku {
        let mut board = [0; 81];
        s.chars().zip(board.iter_mut()).for_each(|(c, u)| {
            if c.is_digit(10) {
                *u = c.to_digit(10).unwrap() as u8;
            }
        });
        Sudoku { board }
    }

    fn rows(&self) -> impl Iterator<Item=&[u8]> {
        self.board.chunks(9)
    }

    fn cols(&self) -> impl Iterator<Item=&[u8]> {
        self.board
    }

    fn is_valid(&self) -> bool {
        // Check that all cells are in range [0, 9]
        if self.board.iter().any(|cell| *cell > 9) {
            return false;
        }

        let mut seen = [false; 10];
        if self.rows().any(|row| {
            // Reset the bit vector
            seen.fill(false);

            // Check that each row contains no duplicates
            row.iter().any(|cell| match *cell {
                0 => false,
                x if seen[x as usize] => true,
                _ => {
                    seen[*cell as usize] = true;
                    false
                }
            })
        }) {
            return false;
        }

        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] != 0 {
                    if seen[self.board[i][j] as usize] {
                        return false;
                    }
                    seen[self.board[i][j] as usize] = true;
                }
            }
            seen.iter_mut().for_each(|b| *b = false);
        }

        for j in 0..9 {
            for i in 0..9 {
                if self.board[i][j] != 0 {
                    if seen[self.board[i][j] as usize] {
                        return false;
                    }
                    seen[self.board[i][j] as usize] = true;
                }
            }
            seen.iter_mut().for_each(|b| *b = false);
        }

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        if self.board[i * 3 + k][j * 3 + l] != 0 {
                            if seen[self.board[i * 3 + k][j * 3 + l] as usize] {
                                return false;
                            }
                            seen[self.board[i * 3 + k][j * 3 + l] as usize] = true;
                        }
                    }
                }
                seen.iter_mut().for_each(|b| *b = false);
            }
        }
        true
    }

    fn is_solved(&self) -> bool {
        self.is_valid()
            && self
                .board
                .iter()
                .all(|elem|  *elem != 0)
    }
}

impl From<&str> for Sudoku {
    fn from(s: &str) -> Sudoku {
        Sudoku::deserialize(s)
    }
}

impl From<[u8; 81]> for Sudoku {
    fn from(board: [u8; 81]) -> Sudoku {
        Sudoku { board }
    }
}

impl From<Sudoku> for String {
    fn from(sudoku: Sudoku) -> String {
        sudoku.serialize()
    }
}

impl From<Sudoku> for [u8; 81] {
    fn from(sudoku: Sudoku) -> [u8; 81] {
        sudoku.board
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, row) in self.rows().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    s.push('.')
                } else {
                    s.push_str(&cell.to_string())
                }
                if j == 2 || j == 5 {
                    s.push('|');
                }
            }
            s.push('\n');
            if i == 2 || i == 5 {
                s.push_str("------+-------+------\n");
            }
        }
        write!(f, "{}", s)
    }
}
