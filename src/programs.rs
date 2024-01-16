use std::cmp;
use rand::prelude::SliceRandom;
use crate::game::{Color, Move};

pub fn take_back_once_prisoner(last_moves: &[Move]) -> Color {
    if let Some(last_move) = last_moves.last() {
        if last_move.0 == Color::Green && last_move.1 == Color::Red {
            return Color::Red;
        }
    }

    Color::Green
}

pub fn friendly(_last_moves: &[Move]) -> Color {
    Color::Green
}

pub fn evil(_last_moves: &[Move]) -> Color {
    Color::Red
}

pub fn blue(_last_moves: &[Move]) -> Color {
    Color::Blue
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

pub fn greedy_blue_and_friendly(last_moves: &[Move]) -> Color {
    let blue_count = last_moves.iter()
        .fold((0, 0), |acc, m| {
            match m {
                (Color::Blue, Color::Blue) => (acc.0 + 1, acc.1 + 1),
                (Color::Blue, _) => (acc.0 + 1, acc.1),
                (_, Color::Blue) => (acc.0, acc.1 + 1),
                _ => acc,
            }
        });

    if blue_count.0 > blue_count.1 {
        Color::Green
    } else {
        Color::Blue
    }
}

pub fn greedy_blue_and_evil(last_moves: &[Move]) -> Color {
    let blue_count = last_moves.iter()
        .fold((0, 0), |acc, m| {
            match m {
                (Color::Blue, Color::Blue) => (acc.0 + 1, acc.1 + 1),
                (Color::Blue, _) => (acc.0 + 1, acc.1),
                (_, Color::Blue) => (acc.0, acc.1 + 1),
                _ => acc,
            }
        });

    if blue_count.0 > blue_count.1 {
        Color::Red
    } else {
        Color::Blue
    }
}

/// `try_to_guess` will try to make the "best" response based only on what the opponent has played the most of.
pub fn try_to_guess(last_moves: &[Move]) -> Color {
    let mut belief = (0, 0, 0);
    last_moves.iter()
        .for_each(|x| match x.1 {
            Color::Red => belief.0 += 1,
            Color::Green => belief.2 += 1,
            Color::Blue => belief.1 += 1,
        });
    
    // OMM - Opponent's Most likely Move
    match belief.0.cmp(&belief.1) {
        // OMM is Blue, Green and Blue, or Green
        // "Best" response is Green
        cmp::Ordering::Less => Color::Green,
        cmp::Ordering::Equal => match belief.1.cmp(&belief.2) {
            // OMM is Blue
            // "Best" response is Green
            cmp::Ordering::Less => Color::Green,
            // OMM is Red and Blue and Green
            // "Best" response is Red but
            // increasing the Blue count may not be bad either
            cmp::Ordering::Equal => *[Color::Red, Color::Blue].choose(&mut rand::thread_rng()).unwrap(),
            // OMM is Red and Green
            // "Best" response is Red but
            // increasing the Blue count may not be bad either
            cmp::Ordering::Greater => *[Color::Red, Color::Blue].choose(&mut rand::thread_rng()).unwrap(),
        }
        cmp::Ordering::Greater => match belief.0.cmp(&belief.2) {
            // OMM is Blue
            // "Best" response is Green
            cmp::Ordering::Less => Color::Green,
            // OMM is Red and Blue
            // "Best" response is Red but
            // increasing the Blue count may not be bad either
            cmp::Ordering::Equal => *[Color::Red, Color::Blue].choose(&mut rand::thread_rng()).unwrap(),
            // OMM is Red
            // "Best" response is Red but
            // increasing the Blue count may not be bad either
            cmp::Ordering::Greater => *[Color::Red, Color::Blue].choose(&mut rand::thread_rng()).unwrap(),
        }
    }
}

pub fn random(_last_moves: &[Move]) -> Color {
    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}

pub fn greedy_if_winning_else_friendly(last_moves: &[Move]) -> Color {
    let scores = last_moves.iter()
        .fold((0, 0), |scores_acc, m| {
            match m {
                (Color::Red, Color::Red) => (scores_acc.0 + 1, scores_acc.1 + 1),
                (Color::Red, Color::Green) => (scores_acc.0 + 3, scores_acc.1),
                (Color::Green, Color::Red) => (scores_acc.0, scores_acc.1 + 3),
                (Color::Green, Color::Green) => (scores_acc.0 + 2, scores_acc.1 + 2),
                (Color::Blue, Color::Blue) => scores_acc,
                (Color::Blue, _) => (scores_acc.0 - 1, scores_acc.1 + 1),
                (_, Color::Blue) => (scores_acc.0 + 1, scores_acc.1 - 1),
            }
        });
    
    if scores.0 > scores.1 {
        return Color::Blue;
    }

    Color::Green
}

pub fn greedy_if_winning_else_evil(last_moves: &[Move]) -> Color {
    let scores = last_moves.iter()
        .fold((0, 0), |scores_acc, m| {
            match m {
                (Color::Red, Color::Red) => (scores_acc.0 + 1, scores_acc.1 + 1),
                (Color::Red, Color::Green) => (scores_acc.0 + 3, scores_acc.1),
                (Color::Green, Color::Red) => (scores_acc.0, scores_acc.1 + 3),
                (Color::Green, Color::Green) => (scores_acc.0 + 2, scores_acc.1 + 2),
                (Color::Blue, Color::Blue) => scores_acc,
                (Color::Blue, _) => (scores_acc.0 - 1, scores_acc.1 + 1),
                (_, Color::Blue) => (scores_acc.0 + 1, scores_acc.1 - 1),
            }
        });
    
    if scores.0 > scores.1 {
        return Color::Blue;
    }

    Color::Red
}

pub fn greedy_if_2x_score_else_friendly(last_moves: &[Move]) -> Color {
    let scores = last_moves.iter()
        .fold((0, 0), |scores_acc, m| {
            match m {
                (Color::Red, Color::Red) => (scores_acc.0 + 1, scores_acc.1 + 1),
                (Color::Red, Color::Green) => (scores_acc.0 + 3, scores_acc.1),
                (Color::Green, Color::Red) => (scores_acc.0, scores_acc.1 + 3),
                (Color::Green, Color::Green) => (scores_acc.0 + 2, scores_acc.1 + 2),
                (Color::Blue, Color::Blue) => scores_acc,
                (Color::Blue, _) => (scores_acc.0 - 1, scores_acc.1 + 1),
                (_, Color::Blue) => (scores_acc.0 + 1, scores_acc.1 - 1),
            }
        });
    
    if last_moves.len() > 0 && scores.0 >= scores.1 * 2 {
        return Color::Blue;
    }

    Color::Green
}

pub fn greedy_if_2x_score_else_evil(last_moves: &[Move]) -> Color {
    let scores = last_moves.iter()
        .fold((0, 0), |scores_acc, m| {
            match m {
                (Color::Red, Color::Red) => (scores_acc.0 + 1, scores_acc.1 + 1),
                (Color::Red, Color::Green) => (scores_acc.0 + 3, scores_acc.1),
                (Color::Green, Color::Red) => (scores_acc.0, scores_acc.1 + 3),
                (Color::Green, Color::Green) => (scores_acc.0 + 2, scores_acc.1 + 2),
                (Color::Blue, Color::Blue) => scores_acc,
                (Color::Blue, _) => (scores_acc.0 - 1, scores_acc.1 + 1),
                (_, Color::Blue) => (scores_acc.0 + 1, scores_acc.1 - 1),
            }
        });
    
    if last_moves.len() > 0 && scores.0 >= scores.1 * 2 {
        return Color::Blue;
    }

    Color::Red
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendly_test() {
        assert!(friendly(&[]) == Color::Green);
        assert!(friendly(&[(Color::Green, Color::Red)]) == Color::Green);
    }

    #[test]
    fn evil_test() {
        assert!(evil(&[]) == Color::Red);
        assert!(evil(&[(Color::Red, Color::Green)]) == Color::Red);
    }

    #[test]
    fn greedy_blue_test() {
        assert!(blue(&[]) == Color::Blue);
        assert!(blue(&[(Color::Blue, Color::Red)]) == Color::Blue);
    }

    #[test]
    fn tit_for_tat_prisoner_test() {
        assert!(tit_for_tat_prisoner(&[]) == Color::Green);
        assert!(tit_for_tat_prisoner(&[(Color::Green, Color::Red)]) == Color::Red);
        assert!(tit_for_tat_prisoner(&[(Color::Green, Color::Green)]) == Color::Green);
    }

    #[test]
    fn take_back_prisoner_test() {
        assert!(take_back_once_prisoner(&[]) == Color::Green);
        assert!(take_back_once_prisoner(&[(Color::Green, Color::Green)]) == Color::Green);
        assert!(take_back_once_prisoner(&[(Color::Green, Color::Red)]) == Color::Red);
        assert!(take_back_once_prisoner(&[(Color::Red, Color::Green)]) == Color::Green);
        assert!(take_back_once_prisoner(&[(Color::Red, Color::Red)]) == Color::Green);
    }

    #[test]
    fn tit_for_two_tats_prisoner_test() {
        assert!(tit_for_two_tats_prisoner(&[]) == Color::Green);
        assert!(tit_for_two_tats_prisoner(&[(Color::Green, Color::Red)]) == Color::Green);
        assert!(tit_for_two_tats_prisoner(&[(Color::Green, Color::Red), (Color::Green, Color::Red)]) == Color::Red);
    }

    #[test]
    fn greedy_blue_and_evil_test() {
        assert!(greedy_blue_and_evil(&[]) == Color::Blue);
        assert!(greedy_blue_and_evil(&[(Color::Blue, Color::Red)]) == Color::Red);
        assert!(greedy_blue_and_evil(&[(Color::Blue, Color::Blue), (Color::Blue, Color::Green)]) == Color::Red);
        assert!(greedy_blue_and_evil(&[(Color::Blue, Color::Red), (Color::Red, Color::Blue)]) == Color::Blue);
    }

    #[test]
    fn greedy_blue_and_friendly_test() {
        assert!(greedy_blue_and_friendly(&[]) == Color::Blue);
        assert!(greedy_blue_and_friendly(&[(Color::Blue, Color::Red)]) == Color::Green);
        assert!(greedy_blue_and_friendly(&[(Color::Blue, Color::Blue), (Color::Blue, Color::Green)]) == Color::Green);
        assert!(greedy_blue_and_friendly(&[(Color::Blue, Color::Red), (Color::Red, Color::Blue)]) == Color::Blue);
    }

    #[test]
    fn greedy_if_winning_else_friendly_test() {
        assert!(greedy_if_winning_else_friendly(&[]) == Color::Green);
        assert!(greedy_if_winning_else_friendly(&[(Color::Green, Color::Red)]) == Color::Green);
        assert!(greedy_if_winning_else_friendly(&[(Color::Green, Color::Blue), (Color::Blue, Color::Green)]) == Color::Green);
        assert!(greedy_if_winning_else_friendly(&[(Color::Green, Color::Green), (Color::Green, Color::Blue)]) == Color::Blue);
    }

    #[test]
    fn greedy_if_winning_else_evil_test() {
        assert!(greedy_if_winning_else_evil(&[]) == Color::Red);
        assert!(greedy_if_winning_else_evil(&[(Color::Red, Color::Red)]) == Color::Red);
        assert!(greedy_if_winning_else_evil(&[(Color::Red, Color::Blue), (Color::Blue, Color::Green)]) == Color::Red);
        assert!(greedy_if_winning_else_evil(&[(Color::Red, Color::Green), (Color::Blue, Color::Blue)]) == Color::Blue);
    }

    #[test]
    fn greedy_if_2x_score_else_friendly_test() {
        assert!(greedy_if_2x_score_else_friendly(&[]) == Color::Green);
        assert!(greedy_if_2x_score_else_friendly(&[(Color::Green, Color::Blue)]) == Color::Blue);
        assert!(greedy_if_2x_score_else_friendly(&[(Color::Green, Color::Green), (Color::Green, Color::Blue)]) == Color::Blue);
    }

    #[test]
    fn greedy_if_2x_score_else_evil_test() {
        assert!(greedy_if_2x_score_else_evil(&[]) == Color::Red);
        assert!(greedy_if_2x_score_else_evil(&[(Color::Red, Color::Blue)]) == Color::Blue);
        assert!(greedy_if_2x_score_else_evil(&[(Color::Red, Color::Red), (Color::Red, Color::Blue)]) == Color::Blue);
    }
}