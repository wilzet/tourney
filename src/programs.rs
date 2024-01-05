use crate::game::{Ply, Move};

pub fn take_back(last_moves: &[Move]) -> Ply {
    if let Some(last_move) = last_moves.last() {
        if last_move.0 == Ply::Green && last_move.1 == Ply::Red {
            return Ply::Red;
        }
    }

    Ply::Green
}

pub fn friendly(_last_moves: &[Move]) -> Ply {
    Ply::Green
}

pub fn evil(_last_moves: &[Move]) -> Ply {
    Ply::Red
}

pub fn tit_for_tat(last_moves: &[Move]) -> Ply {
    if let Some(last_move) = last_moves.last() {
        if last_move.1 == Ply::Red {
            return Ply::Red;
        }
    }

    Ply::Green
}

pub fn tit_for_two_tats(last_moves: &[Move]) -> Ply {
    if last_moves.len() < 2 {
        return Ply::Green;
    }

    if let Some(two_last_moves) = last_moves.get(last_moves.len()-2..) {
        if two_last_moves[0].1 == Ply::Red && two_last_moves[1].1 == Ply::Red {
            return Ply::Red;
        }
    }

    Ply::Green
}

pub fn greedy_blue_and_friendly(last_moves: &[Move]) -> Ply {
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
        Ply::Green
    } else {
        Ply::Blue
    }
}

pub fn greedy_blue_and_evil(last_moves: &[Move]) -> Ply {
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
        Ply::Red
    } else {
        Ply::Blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendly_prisoner_test() {
        assert!(friendly(&[]) == Ply::Green);
        assert!(friendly(&[(Ply::Green, Ply::Red)]) == Ply::Green);
    }

    #[test]
    fn evil_prisoner_test() {
        assert!(evil(&[]) == Ply::Red);
        assert!(evil(&[(Ply::Red, Ply::Green)]) == Ply::Red);
    }

    #[test]
    fn tit_for_tat_prisoner_test() {
        assert!(tit_for_tat(&[]) == Ply::Green);
        assert!(tit_for_tat(&[(Ply::Green, Ply::Red)]) == Ply::Red);
        assert!(tit_for_tat(&[(Ply::Green, Ply::Green)]) == Ply::Green);
    }

    #[test]
    fn take_back_prisoner_test() {
        assert!(take_back(&[]) == Ply::Green);
        assert!(take_back(&[(Ply::Green, Ply::Green)]) == Ply::Green);
        assert!(take_back(&[(Ply::Green, Ply::Red)]) == Ply::Red);
        assert!(take_back(&[(Ply::Red, Ply::Green)]) == Ply::Green);
        assert!(take_back(&[(Ply::Red, Ply::Red)]) == Ply::Green);
    }

    #[test]
    fn tit_for_two_tats_prisoner_test() {
        assert!(tit_for_two_tats(&[]) == Ply::Green);
        assert!(tit_for_two_tats(&[(Ply::Green, Ply::Red)]) == Ply::Green);
        assert!(tit_for_two_tats(&[(Ply::Green, Ply::Red), (Ply::Green, Ply::Red)]) == Ply::Red);
    }

    #[test]
    fn greedy_blue_and_evil_test() {
        assert!(greedy_blue_and_evil(&[]) == Ply::Blue);
        assert!(greedy_blue_and_evil(&[(Ply::Blue, Ply::Red)]) == Ply::Red);
        assert!(greedy_blue_and_evil(&[(Ply::Blue, Ply::Blue), (Ply::Blue, Ply::Green)]) == Ply::Red);
        assert!(greedy_blue_and_evil(&[(Ply::Blue, Ply::Red), (Ply::Red, Ply::Blue)]) == Ply::Blue);
    }

    #[test]
    fn greedy_blue_and_friendly_test() {
        assert!(greedy_blue_and_friendly(&[]) == Ply::Blue);
        assert!(greedy_blue_and_friendly(&[(Ply::Blue, Ply::Red)]) == Ply::Green);
        assert!(greedy_blue_and_friendly(&[(Ply::Blue, Ply::Blue), (Ply::Blue, Ply::Green)]) == Ply::Green);
        assert!(greedy_blue_and_friendly(&[(Ply::Blue, Ply::Red), (Ply::Red, Ply::Blue)]) == Ply::Blue);
    }
}