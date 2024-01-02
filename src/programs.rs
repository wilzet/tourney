use crate::game::Move;

pub fn take_back(last_moves: &Vec::<(Move, Move)>) -> Move {
    if let Some(last_move) = last_moves.last() {
        if last_move.0 == Move::GREEN && last_move.1 != Move::GREEN {
            return Move::RED;
        }
    }

    Move::GREEN
}

pub fn friendly(_last_moves: &Vec::<(Move, Move)>) -> Move {
    Move::GREEN
}

pub fn evil(_last_moves: &Vec::<(Move, Move)>) -> Move {
    Move::RED
}

pub fn tit_for_tat(last_moves: &Vec::<(Move, Move)>) -> Move {
    if let Some(last_move) = last_moves.last() {
        if last_move.1 != Move::GREEN {
            return Move::RED;
        }
    }

    Move::GREEN
}

pub fn tit_for_two_tats(last_moves: &Vec::<(Move, Move)>) -> Move {
    if last_moves.len() < 2 {
        return Move::GREEN;
    }

    if let Some(two_last_moves) = last_moves.get(last_moves.len()-2..) {
        if two_last_moves[0].1 == Move::RED && two_last_moves[1].1 == Move::RED {
            return Move::RED;
        }
    }

    Move::GREEN
}