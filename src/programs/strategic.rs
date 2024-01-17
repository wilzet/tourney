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