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