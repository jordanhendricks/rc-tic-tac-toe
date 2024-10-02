use clap::Parser;
use std::io;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// number of rows/cols on the board
    #[arg(short, long, default_value = "3")]
    size: usize,

    /// single player mode
    #[clap(short, long, default_value = "false")]
    solo: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Piece {
    X,
    O,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Player {
    One,
    Two,
    Cpu,
}

struct Board {
    height: usize,
    width: usize,
    pieces: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let mut pieces = Vec::new();
        for _i in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(None);
            }
            pieces.push(row);
        }

        Self {
            height,
            width,
            pieces,
        }
    }

    /// Pretty print the board for terminal play.
    pub fn print(&self) {
        fn print_sep(w: usize) {
            for j in 0..w {
                if j == 0 {
                    print!("    +");
                }
                print!("---+");
            }
            println!("");
        }

        for j in 0..self.width {
            if j == 0 {
                print!("    ");
            }
            print!("  {} ", j);
        }
        println!("");

        print_sep(self.width);

        for i in 0..self.height {
            for j in 0..self.width {
                if j == 0 {
                    print!("{}   |", i);
                }

                match self.pieces[i][j] {
                    Some(p) => {
                        print!(" {:?} |", p);
                    }
                    None => {
                        print!("   |");
                    }
                }
            }
            println!("");

            print_sep(self.width);
        }
    }

    /// Attempts to place the piece, assuming there isn't a piece there already.
    /// Returns true if the piece could be placed; false otherwise.
    pub fn place(&mut self, row: usize, col: usize, t: Piece) -> bool {
        assert!(
            row < self.height,
            "row selection {} out of range of height {}",
            row,
            self.height
        );
        assert!(
            col < self.width,
            "col selection {} out of range of width {}",
            col,
            self.width
        );

        match self.pieces[row][col] {
            Some(_) => false,
            None => {
                self.pieces[row][col] = Some(t);
                true
            }
        }
    }

    fn check_win_row(&self, row: usize, p: Piece) -> bool {
        for c in 0..self.width {
            let v = self.pieces[row][c];
            if v.is_none() || v.unwrap() != p {
                return false;
            }
        }
        true
    }

    fn check_win_col(&self, col: usize, p: Piece) -> bool {
        for r in 0..self.height {
            let v = self.pieces[r][col];
            if v.is_none() || v.unwrap() != p {
                return false;
            }
        }
        true
    }

    fn check_win_diag_left(&self, p: Piece) -> bool {
        assert!(self.height == self.width, "diagonal calculation assumes board is square");

        let side = self.height;
        for i in 0..side {
            let v = self.pieces[i][i];
            if v.is_none() || v.unwrap() != p {
                return false;
            }
        }
        true
    }

    fn check_win_diag_right(&self, p: Piece) -> bool {
        assert!(self.height == self.width, "diagonal calculation assumes board is square");

        let side = self.height;
        for i in 0..side {
            let v = self.pieces[self.height - i - 1][i];
            if v.is_none() || v.unwrap() != p {
                return false;
            }
        }
        true
    }

    /// Determines if the given piece has won. Returns true if so; false otherwise..
    pub fn check_win_condition(&self, p: Piece) -> bool {
        // Check for a full row.
        for i in 0..self.height {
            if self.check_win_row(i, p) {
                return true;
            }
        }

        // Check for a full column.
        for j in 0..self.height {
            if self.check_win_col(j, p) {
                return true;
            }
        }

        // Check diagonals.
        if self.check_win_diag_left(p) || self.check_win_diag_right(p) {
            return true;
        }

        false
    }

    /// Returns true if there are no more possible moves on the board.
    pub fn is_full(&self) -> bool {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.pieces[i][j].is_none() {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let args = Args::parse();

    // Game intro and number of player selection
    println!("");
    println!("{:^80}", "TIC TAC TOE: INTERACTIVE TERMINAL VERSION");
    println!("");

    println!("BOARD SIZE: {}x{}", args.size, args.size);
    print!("MODE: ");
    if args.solo {
        println!("single player");
    } else {
        println!("two player");
    }
    println!("");

    // Start the game loop
    let mut board = Board::new(args.size, args.size);
    let mut cur_player = Player::One;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));

        let mut rs = String::new();
        let mut cs = String::new();

        board.print();
        let p = if cur_player == Player::One {
            Piece::X
        } else {
            Piece::O
        };

        println!("");
        println!("TURN: Player {:?} (\"{:?}\")", cur_player, p);
        println!("");

        println!("select row:");
        io::stdin().read_line(&mut rs).expect("failed to read row");

        let rt = rs.trim().parse();
        if rt.is_err() {
            println!("");
            println!("ERROR: invalid row");
            println!("");
            continue;
        }
        let row = rt.unwrap();
        if row >= board.height {
            println!("");
            println!("ERROR: row out of range");
            println!("");
            continue;
        }

        println!("select col:");
        io::stdin().read_line(&mut cs).unwrap();

        let ct = cs.trim().parse();
        if ct.is_err() {
            println!("");
            println!("ERROR: invalid col");
            println!("");
            continue;
        }
        let col = ct.unwrap();
        if col >= board.width {
            println!("");
            println!("ERROR: col out of range");
            println!("");
            continue;
        }

        if !board.place(row, col, p) {
            // There's a piece already there.
            println!("");
            println!("ERROR: space at row {}, col {} already occupied", row, col);
            println!("");
            continue;
        }

        // Determine if the game is over.
        if !board.check_win_condition(p) {
            // Check for stalemate
            if board.is_full() {
                board.print();
                println!("");
                println!("GAME OVER: stalemate");
                println!("");
                return;
            }
        } else {
            // Game over!
            board.print();
            println!("");
            println!("GAME OVER: Player {:?} wins!", cur_player);
            return;
        }

        cur_player = match (cur_player, args.solo) {
            (Player::One, true) => Player::Cpu,
            (Player::One, false) => Player::Two,
            (Player::Two, false) => Player::One,
            (Player::Cpu, true) => Player::One,
            (Player::Cpu, false) => {
                unreachable!("two-player mode without CPU is not valid");
            }
            (Player::Two, true) => {
                unreachable!("two-player mode with CPU is not valid");
            }
        };
    }
}
