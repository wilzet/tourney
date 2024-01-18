use crate::programs::prelude::*;

/// `try_to_guess` will try to make the "best" response based only on what the opponent has played the most of.
pub fn try_to_guess(last_moves: &[Move]) -> Color {
    let (red_count, green_count, blue_count) = last_moves.iter()
        .fold((0, 0, 0), |(r, g, b), m| match m.1 {
            Color::Red => (r + 1, g, b),
            Color::Green => (r, g + 1, b),
            Color::Blue => (r, g, b + 1),
        });
    
    // OMM - Opponent's Most likely Move
    match red_count.cmp(&green_count) {
        // OMM is Blue, Green and Blue, or Green
        // "Best" response is Green
        cmp::Ordering::Less => Color::Green,
        cmp::Ordering::Equal => match green_count.cmp(&blue_count) {
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
        cmp::Ordering::Greater => match red_count.cmp(&blue_count) {
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

fn some_greed_and_match_opponent(last_moves: &[Move], greedy_rounds: usize) -> Color {
    // Try to be greedy early
    if last_moves.len() < greedy_rounds {
        let blue_count = last_moves.iter()
            .fold((0, 0), |acc, m| {
                match m {
                    (Color::Blue, Color::Blue) => (acc.0 + 1, acc.1 + 1),
                    (Color::Blue, _) => (acc.0 + 1, acc.1),
                    (_, Color::Blue) => (acc.0, acc.1 + 1),
                    _ => acc,
                }
            });

        // Want blue_count difference of 2
        if blue_count.0 - 1 <= blue_count.1 {
            return Color::Blue;
        }
    }

    let (red_count, green_count, blue_count) = last_moves.iter()
        .fold((0, 0, 0), |(r, g, b), m| match m.1 {
            Color::Red => (r + 1, g, b),
            Color::Green => (r, g + 1, b),
            Color::Blue => (r, g, b + 1),
        });

    // If opponent chooses mostly green or red, choose that too.
    // Else compare with blue, and choose randomly
    match red_count.cmp(&green_count) {
        cmp::Ordering::Less => Color::Green,
        cmp::Ordering::Equal => match red_count.cmp(&blue_count) {
            cmp::Ordering::Equal => *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap(),
            _ => *[Color::Red, Color::Green].choose(&mut rand::thread_rng()).unwrap(),
        }
        cmp::Ordering::Greater => Color::Red,
    }
}

pub fn greed_first_15(last_moves: &[Move]) -> Color {
    some_greed_and_match_opponent(last_moves, 15)
}