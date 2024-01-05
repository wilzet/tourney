#[derive(PartialEq, Clone, Copy)]
pub enum Ply {
    Red,
    Green,
    Blue,
}

pub type Move = (Ply, Ply);
type Program = fn(&[Move]) -> Ply;

/// # Player
/// Represents a player program
#[derive(Clone)]
pub struct Player {
    name: Option<String>,
    program: Option<Program>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: None,
            program: None,
        }
    }

    pub fn with_name_and_program(name: String, program: Program) -> Player {
        Player::new().set_name(name).set_program(program)
    }

    pub fn set_name(mut self, name: String) -> Player {
        self.name = Some(name);
        self
    }

    pub fn set_program(mut self, program: Program) -> Player {
        self.program = Some(program);
        self
    }

    pub fn get_name(&self) -> String {
        match &self.name {
            Some(name) => String::from(name),
            None => String::from(""),
        }
    }

    fn make_move(&self, last_move: &[Move]) -> Ply {
        let func = self.program.unwrap();
        func(last_move)
    }
}

pub fn play(player1: Player, player2: Player, rounds: u32) -> (i32, i32) {
    let mut scores = (0, 0);
    let mut last_moves = Vec::new();
    for _ in 0..rounds {
        let player1_move = player1.make_move(&last_moves);

        let last_moves_swapped = last_moves.iter()
            .map(|moves| (moves.1, moves.0))
            .collect::<Vec<Move>>();
        let player2_move = player2.make_move(&last_moves_swapped);

        let this_move = (player1_move, player2_move);
        match this_move {
            (Ply::Red, Ply::Red) => {
                scores.0 += 1;
                scores.1 += 1;
            }
            (Ply::Red, Ply::Green) => {
                scores.0 += 3;
            }
            (Ply::Green, Ply::Red) => {
                scores.1 += 3;
            }
            (Ply::Green, Ply::Green) => {
                scores.0 += 2;
                scores.1 += 2;
            }
            (Ply::Blue, Ply::Blue) => (),
            (Ply::Blue, _) => {
                scores.0 -= 1;
                scores.1 += 1;
            }
            (_, Ply::Blue) => {
                scores.0 += 1;
                scores.1 -= 1;
            }
        };

        last_moves.push(this_move);
    }

    let blue_count = last_moves.iter()
        .fold((0, 0), |acc, m| {
            match m {
                (Ply::Blue, Ply::Blue) => (acc.0 + 1, acc.1 + 1),
                (Ply::Blue, _) => (acc.0 + 1, acc.1),
                (_, Ply::Blue) => (acc.0, acc.1 + 1),
                _ => acc,
            }
        });

    if blue_count.0 > blue_count.1 {
        (scores.0 * 2, scores.1)
    } else if blue_count.0 < blue_count.1 {
        (scores.0, scores.1 * 2)
    } else {
        scores
    }
}