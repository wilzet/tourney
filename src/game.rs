#[derive(PartialEq, Clone, Copy)]
pub enum Move {
    RED,
    GREEN,
}

/// # Player
/// Represents a player program
#[derive(Clone)]
pub struct Player {
    name: Option<String>,
    program: Option<fn(&Vec::<(Move, Move)>) -> Move>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: None,
            program: None,
        }
    }

    pub fn with_name_and_program(name: String, program: fn(&Vec::<(Move, Move)>) -> Move) -> Player {
        Player::new().set_name(name).set_program(program)
    }

    pub fn set_name(mut self, name: String) -> Player {
        self.name = Some(name);
        self
    }

    pub fn set_program(mut self, program: fn(&Vec::<(Move, Move)>) -> Move) -> Player {
        self.program = Some(program);
        self
    }

    pub fn get_name(&self) -> String {
        match &self.name {
            Some(name) => String::from(name),
            None => String::from(""),
        }
    }

    fn make_move(&self, last_move: &Vec::<(Move, Move)>) -> Move {
        let func = self.program.unwrap();
        func(last_move)
    }
}

pub fn play(player1: Player, player2: Player, rounds: u32) -> (i32, i32) {
    let mut scores = (0, 0);
    let mut last_moves = vec![];
    for _ in 0..rounds {
        let player1_move = player1.make_move(&last_moves);

        let last_moves_swapped = last_moves.iter()
            .map(|moves| (moves.1, moves.0))
            .collect();
        let player2_move = player2.make_move(&last_moves_swapped);
        
        last_moves.push((player1_move, player2_move));

        match last_moves.last() {
            Some((Move::RED, Move::RED)) => {
                scores.0 += 1;
                scores.1 += 1;
            }
            Some((Move::RED, Move::GREEN)) => {
                scores.0 += 5;
            }
            Some((Move::GREEN, Move::RED)) => {
                scores.1 += 5;
            }
            Some((Move::GREEN, Move::GREEN)) => {
                scores.0 += 3;
                scores.1 += 3;
            }
            _ => (),
        };
    }

    scores
}