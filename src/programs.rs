mod prelude {
    pub use std::cmp;
    pub use rand::prelude::*;
    pub use crate::game::{
        Color,
        Move,
        calculate_scores,
    };
}

pub mod all {
    pub use crate::programs::{
        chat_gpt::*,
        prisoners::*,
        greedy::*,
        simple::*,
        strategic::*,
    };
}

pub mod chat_gpt;
pub mod prisoners;
pub mod greedy;
pub mod simple;
pub mod strategic;

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use super::all::*;

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
    fn chat_gpt_adaptive_test() {
        assert!(chat_gpt_adaptive(&[(Color::Green, Color::Red)]) == Color::Blue);
        assert!(chat_gpt_adaptive(&[(Color::Green, Color::Green)]) == Color::Red);
        assert!(chat_gpt_adaptive(&[(Color::Green, Color::Blue)]) == Color::Green);
    }

    #[test]
    fn chat_gpt_proactive_test() {
        assert!(chat_gpt_proactive(&[(Color::Green, Color::Green)]) == Color::Red);
        assert!(chat_gpt_proactive(&[(Color::Green, Color::Blue)]) == Color::Green);
    }

    #[test]
    fn cooperate_until_defection_test() {
        assert!(cooperate_until_defection(&[(Color::Green, Color::Green)]) == Color::Green);
        assert!(cooperate_until_defection(&[(Color::Green, Color::Red)]) != Color::Green);
    }

    #[test]
    fn greedy_if_winning_else_random_test() {
        assert!(greedy_if_winning_else_random(&[(Color::Red, Color::Green)]) == Color::Blue);
    }

    #[test]
    fn greedy_if_2x_score_else_random_test() {
        assert!(greedy_if_2x_score_else_random(&[(Color::Red, Color::Blue)]) == Color::Blue);
    }

    #[test]
    fn copy_test() {
        assert!(copy(&[(Color::Red, Color::Red)]) == Color::Red);
        assert!(copy(&[(Color::Red, Color::Green)]) == Color::Green);
        assert!(copy(&[(Color::Red, Color::Blue)]) == Color::Blue);
    }

    #[test]
    fn smarter_copy_test() {
        assert!(copy(&[(Color::Red, Color::Red)]) == Color::Red);
        assert!(copy(&[(Color::Red, Color::Green)]) == Color::Green);
        assert!(copy(&[(Color::Red, Color::Blue)]) != Color::Red);
    }
}