//! `game` contains all necessities to play the game (described [here](https://github.com/wilzet/tourney)).

/// A ply is a turn taken by one [player](Player). Two plies make a [move](Move).
#[derive(PartialEq, Clone, Copy)]
pub enum Ply {
    /// Non-cooperative.
    Red,
    /// Cooperative.
    Green,
    /// The player with the most blue plies gets their score doubled at the end of the game.
    Blue,
}

/// A move is a pair of each players' [ply](Ply).
pub type Move = (Ply, Ply);

/// The type defintion for a [player program](Player).
pub type Program = fn(&[Move]) -> Ply;

/// Represents a player program.
/// 
/// To create a player that can play the game, a program is needed.
/// A name that increases readability and ease of identification may be added but is not necessary.
#[derive(Clone)]
pub struct Player {
    name: Option<String>,
    program: Program,
}

impl Player {
    /// Create a new player program.
    /// 
    /// # Arguments
    /// 
    /// * `program` - A program with type defintion according to [`Program`] is able to play the game.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tourney::game::{Player, Ply, Move};
    /// 
    /// // Create a player program
    /// fn example_tit_for_tat_program(last_moves: &[Move]) -> Ply {
    ///     if let Some(last_move) = last_moves.last() {
    ///         if last_move.1 == Ply::Red {
    ///             return Ply::Red;
    ///         }
    ///     }
    ///
    ///     Ply::Green
    /// }
    /// 
    /// // Create a player
    /// let player = Player::new(example_tit_for_tat_program);
    /// ```
    pub fn new(program: Program) -> Player {
        Player {
            name: None,
            program,
        }
    }

    /// Create a new player program with a name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - An identifying name for the player.
    /// * `program` - A program with type defintion according to [`Program`] is able to play the game.
    /// 
    /// # Examples
    /// 
    /// View [`new`](Player::new) for a full example of a player program.
    /// ```
    /// # use tourney::game::{Player, Ply, Move};
    /// #
    /// # fn example_program(last_moves: &[Move]) -> Ply {
    /// #     Ply::Green
    /// # }
    /// // Create a player with a name
    /// let player = Player::with_name("Player Name", example_program);
    /// ```
    pub fn with_name(name: &str, program: Program) -> Player {
        Player::new(program).set_name(name)
    }

    /// Set the name of the player program.
    ///     
    /// # Arguments
    /// 
    /// * `name` - An identifying name for the player.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use tourney::game::{Player, Ply, Move};
    /// #
    /// # fn example_program(last_moves: &[Move]) -> Ply {
    /// #     Ply::Green
    /// # }
    /// // Create a player with a name
    /// let player = Player::with_name("Player Name", example_program);
    /// 
    /// // Change the player's name
    /// let player = player.set_name("Another name");
    /// 
    /// assert_eq!(player.get_name(), "Another name");
    pub fn set_name(mut self, name: &str) -> Player {
        self.name = Some(String::from(name));
        self
    }

    /// Get the name of the player program.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use tourney::game::{Player, Ply, Move};
    /// #
    /// # fn example_program(last_moves: &[Move]) -> Ply {
    /// #     Ply::Green
    /// # }
    /// // Create a player
    /// let player = Player::new(example_program);
    /// 
    /// // Set the player's name
    /// let player = player.set_name("A name");
    /// 
    /// assert_eq!(player.get_name(), "A name");
    pub fn get_name(&self) -> &str {
        match &self.name {
            Some(name) => name,
            None => "",
        }
    }

    /// Executes the player program in order to generate a [ply](Ply).
    /// 
    /// # Arguments
    /// 
    /// * `last_moves` - A slice of [moves](Move)
    fn make_move(&self, last_moves: &[Move]) -> Ply {
        (self.program)(last_moves)
    }
}

/// Play the game.
/// 
/// # Arguments
/// 
/// * `player_1` - A [player](Player)
/// * `player_2` - A [player](Player) (may be the same as `player_1`)
/// * `rounds` - The amount of rounds the game goes on for
/// 
/// # Examples
/// 
/// ```
/// use tourney::game::*;
/// 
/// // Create player programs
/// fn example_tit_for_tat_program(last_moves: &[Move]) -> Ply {
///     if let Some(last_move) = last_moves.last() {
///         if last_move.1 == Ply::Red {
///             return Ply::Red;
///         }
///     }
///
///     Ply::Green
/// }
/// 
/// fn example_evil_program(last_moves: &[Move]) -> Ply {
///     Ply::Red
/// }
/// 
/// // Create players
/// let tit_for_tat = Player::new(example_tit_for_tat_program);
/// let evil = Player::new(example_evil_program);
/// 
/// // Play the game
/// let scores = play(evil, tit_for_tat.clone(), 20);
/// assert_eq!(scores, (22, 19));
/// 
/// // Play another game
/// let scores = play(tit_for_tat.clone(), tit_for_tat, 100);
/// assert_eq!(scores, (200, 200));
/// ```
pub fn play(player_1: Player, player_2: Player, rounds: u32) -> (i32, i32) {
    let mut last_moves = Vec::new();
    for _ in 0..rounds {
        let player1_move = player_1.make_move(&last_moves);

        let last_moves_swapped = last_moves.iter()
            .map(|moves| (moves.1, moves.0))
            .collect::<Vec<Move>>();
        let player2_move = player_2.make_move(&last_moves_swapped);

        last_moves.push((player1_move, player2_move));
    }


    let (scores, blue_count) = last_moves.iter()
        .fold(((0, 0), (0, 0)), |(scores_acc, blue_count_acc), m| {
            match m {
                (Ply::Red, Ply::Red) => ((scores_acc.0 + 1, scores_acc.1 + 1), blue_count_acc),
                (Ply::Red, Ply::Green) => ((scores_acc.0 + 3, scores_acc.1), blue_count_acc),
                (Ply::Green, Ply::Red) => ((scores_acc.0, scores_acc.1 + 3), blue_count_acc),
                (Ply::Green, Ply::Green) => ((scores_acc.0 + 2, scores_acc.1 + 2), blue_count_acc),
                (Ply::Blue, Ply::Blue) => (scores_acc, (blue_count_acc.0 + 1, blue_count_acc.1 + 1)),
                (Ply::Blue, _) => ((scores_acc.0 - 1, scores_acc.1 + 1), (blue_count_acc.0 + 1, blue_count_acc.1)),
                (_, Ply::Blue) => ((scores_acc.0 + 1, scores_acc.1 - 1), (blue_count_acc.0, blue_count_acc.1 + 1)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs::greedy_blue_and_friendly;

    fn test_strategy(last_moves: &[Move]) -> Ply {
        if let Some(last_move) = last_moves.last() {
            if last_move.0 == Ply::Green {
                return Ply::Red;
            }

            return Ply::Green;
        }

        Ply::Blue
    }

    #[test]
    fn make_move_test() {
        assert!(Player::new(test_strategy).make_move(&[]) == Ply::Blue);
    }

    #[test]
    fn get_player_name_test() {
        let p = Player::with_name("name", test_strategy);
        assert_eq!(p.get_name(), "name");
        assert_ne!(p.get_name(), Player::new(test_strategy).get_name());
        assert_eq!(Player::new(test_strategy).set_name("name").get_name(), p.get_name());
    }

    #[test]
    fn simple_play_test() {
        let p_1 = Player::with_name("Test", test_strategy);
        let p_2 = Player::new(greedy_blue_and_friendly);

        assert_eq!(play(p_1.clone(), p_2.clone(), 10), (21, 14));
        assert_eq!(play(p_1.clone(), p_1.clone(), 100), (149, 149));
        assert_ne!(p_1.get_name(), p_2.get_name());
    }
}