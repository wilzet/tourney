use crate::programs::prelude::*;

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

pub fn greedy_if_winning_else_random(last_moves: &[Move]) -> Color {
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

    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}

pub fn greedy_if_2x_score_else_random(last_moves: &[Move]) -> Color {
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
    
    if scores.0 >= scores.1 * 2 && scores.0 != scores.1 {
        return Color::Blue;
    }

    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}