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
    let scores = calculate_scores(last_moves);
    
    if scores.0 > scores.1 {
        return Color::Blue;
    }

    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}

pub fn greedy_if_2x_score_else_random(last_moves: &[Move]) -> Color {
    let scores = calculate_scores(last_moves);
    
    if scores.0 >= scores.1 * 2 && scores.0 != scores.1 {
        return Color::Blue;
    }

    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}