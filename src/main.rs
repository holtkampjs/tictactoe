#[derive(Clone, Copy, PartialEq)]
enum Square {
    Empty,
    X,
    O,
}

impl Square {
    fn repr(&self) -> char {
        match self {
            Self::Empty => ' ',
            Self::X => 'X',
            Self::O => 'O',
        }
    }
}

struct Grid {
    player_to_move: Square,
    squares: [Square; 9],
}

impl Grid {
    fn check_for_win(&self) -> bool {
        self.is_line_a_win(0, 1, 2)
            || self.is_line_a_win(3, 4, 5)
            || self.is_line_a_win(6, 7, 8)
            || self.is_line_a_win(0, 3, 6)
            || self.is_line_a_win(1, 4, 7)
            || self.is_line_a_win(2, 5, 8)
            || self.is_line_a_win(0, 4, 8)
            || self.is_line_a_win(2, 4, 6)
    }

    fn display(&self) {
        println!(
            "{}|{}|{}\n-|-|-\n{}|{}|{}\n-|-|-\n{}|{}|{}",
            self.squares[0].repr(),
            self.squares[1].repr(),
            self.squares[2].repr(),
            self.squares[3].repr(),
            self.squares[4].repr(),
            self.squares[5].repr(),
            self.squares[6].repr(),
            self.squares[7].repr(),
            self.squares[8].repr(),
        );
    }

    fn display_turn(&self) {
        if self.player_to_move == Square::X {
            println!("\nIts X's turn: ");
        } else if self.player_to_move == Square::O {
            println!("\nIts O's turn: ");
        }
    }

    fn display_winner(&self) {
        println!("{} wins!!", self.player_to_move.repr());
    }

    fn is_grid_full(&self) -> bool {
        if !self.squares.contains(&Square::Empty) {
            println!("Cats game");
            return true;
        }
        false
    }

    fn is_line_a_win(&self, a: usize, b: usize, c: usize) -> bool {
        self.squares[a] != Square::Empty
            && self.squares[a] == self.squares[b]
            && self.squares[a] == self.squares[c]
    }

    fn new() -> Self {
        Self {
            player_to_move: Square::X,
            squares: [Square::Empty; 9],
        }
    }

    fn next_player(&mut self) {
        if self.player_to_move == Square::X {
            self.player_to_move = Square::O;
        } else {
            self.player_to_move = Square::X;
        }
    }

    fn take_player_move(&mut self, index: usize) -> bool {
        if index < 1 || index > 9 {
            println!("{index} isn't a valid move. Please enter a move between 1 and 9.");
            return false;
        }

        if self.squares[index - 1] != Square::Empty {
            println!("That square is taken, try again");
            return false;
        }

        self.squares[index - 1] = self.player_to_move;
        true
    }
}

fn main() {
    println!("Welcome to Tic Tac Toe!");
    println!(
        "\nTo play, enter numbers 1 thru 9 corresponding to the square you'd like to select\n"
    );
    println!("   |   |   ");
    println!(" 1 | 2 | 3 ");
    println!("   |   |   ");
    println!("---|---|---");
    println!("   |   |   ");
    println!(" 4 | 5 | 6 ");
    println!("   |   |   ");
    println!("---|---|---");
    println!("   |   |   ");
    println!(" 7 | 8 | 9 ");
    println!("   |   |   ");

    let mut grid = Grid::new();

    loop {
        grid.display_turn();

        let player_move = get_player_move();
        if !grid.take_player_move(player_move) {
            continue;
        }

        grid.display();

        if grid.check_for_win() {
            grid.display_winner();
            break;
        }

        if grid.is_grid_full() {
            break;
        }

        grid.next_player();
    }

    println!("Game over");
}

fn get_player_move() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<usize>() {
        Ok(i) => i,
        Err(..) => {
            println!("this did not work: {}", input);
            0
        }
    }
}
