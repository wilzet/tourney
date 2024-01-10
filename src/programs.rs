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

pub fn greedy_blue(_last_moves: &[Move]) -> Color {
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
        assert!(greedy_blue(&[]) == Color::Blue);
        assert!(greedy_blue(&[(Color::Blue, Color::Red)]) == Color::Blue);
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
}