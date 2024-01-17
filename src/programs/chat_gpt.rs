use crate::programs::prelude::*;

pub fn chat_gpt_adaptive(last_moves: &[Move]) -> Color {
    if let Some(opponent_last_move) = last_moves.last().map(|m| m.1) {
        return match opponent_last_move {
            Color::Red => Color::Blue,      // Defect against Red
            Color::Green => Color::Red,     // Exploit Green's cooperation
            Color::Blue => Color::Green,    // Cooperate if opponent chose Blue
        };
    }

    // If no opponent moves recorded, choose randomly
    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}

pub fn chat_gpt_proactive(last_moves: &[Move]) -> Color {
    if let Some(_) = last_moves.last() {
        // Analyze the opponent's historical moves
        let green_count = last_moves.iter().filter(|m| m.1 == Color::Green).count();
        let blue_count = last_moves.iter().filter(|m| m.1 == Color::Blue).count();

        // Proactively choose a color based on opponent's likely strategy
        return if green_count > blue_count {
            // Opponent has a tendency to choose Green
            Color::Red // Exploit by choosing Red
        } else {
            // Opponent has a tendency to choose Blue or mixed strategy
            Color::Green // Cooperate by choosing Green
        };
    }

    // If no opponent moves recorded, choose randomly
    *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap()
}

pub fn chat_gpt_versatile(last_moves: &[Move]) -> Color {
    if last_moves.is_empty() || rand::thread_rng().gen::<f64>() < 0.5 {
        // Introduce randomness or choose randomly if no history
        return *[Color::Red, Color::Green, Color::Blue].choose(&mut rand::thread_rng()).unwrap();
    }

    // Follow the opponent's recent move
    match last_moves.last().map(|m| m.1) {
        Some(Color::Red) => Color::Blue,
        Some(Color::Green) => Color::Red,
        _ => Color::Green,
    }
}