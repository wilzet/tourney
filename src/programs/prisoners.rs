use crate::programs::prelude::*;

pub fn take_back_once_prisoner(last_moves: &[Move]) -> Color {
    if let Some(last_move) = last_moves.last() {
        if last_move.0 == Color::Green && last_move.1 == Color::Red {
            return Color::Red;
        }
    }

    Color::Green
}

pub fn tit_for_tat_prisoner(last_moves: &[Move]) -> Color {
    if let Some(last_move) = last_moves.last() {
        if last_move.1 == Color::Red {
            return Color::Red;
        }
    }

    Color::Green
}

pub fn tit_for_two_tats_prisoner(last_moves: &[Move]) -> Color {
    if last_moves.len() < 2 {
        return Color::Green;
    }

    if let Some(two_last_moves) = last_moves.get(last_moves.len()-2..) {
        if two_last_moves[0].1 == Color::Red && two_last_moves[1].1 == Color::Red {
            return Color::Red;
        }
    }

    Color::Green
}