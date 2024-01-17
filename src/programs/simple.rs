use crate::programs::prelude::*;

pub fn friendly(_last_moves: &[Move]) -> Color {
    Color::Green
}

pub fn evil(_last_moves: &[Move]) -> Color {
    Color::Red
}

pub fn blue(_last_moves: &[Move]) -> Color {
    Color::Blue
}

pub fn random(_last_moves: &[Move]) -> Color {
    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}

pub fn cooperate_until_defection(last_moves: &[Move]) -> Color {
    if last_moves.iter().any(|m| m.1 == Color::Red) {
        return *[Color::Red, Color::Blue].choose(&mut rand::thread_rng()).unwrap();
    }

    Color::Green
}

pub fn copy(last_moves: &[Move]) -> Color {
    if let Some(last_move) = last_moves.last() {
        return last_move.1;
    }

    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}

pub fn smarter_copy(last_moves: &[Move]) -> Color {
    match last_moves.last().map(|m| m.1) {
        Some(Color::Blue) => *[Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap(),
        Some(opponent_move) => opponent_move,
        _ => *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap(),
    }
}