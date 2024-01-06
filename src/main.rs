use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use rand::{prelude::*, distributions};
use tourney::game::{Player, play};
use tourney::programs::*;

const MIN_ROUNDS: u32 = 70;
const MAX_ROUNDS: u32 = 100;

fn main() {
    println!("Program start\n");
    let pool = ThreadPool::with_name("Games".into(), 20);

    let players = vec![
        Player::with_name("Take back 1", take_back),
        Player::with_name("Friendly", friendly),
        Player::with_name("Evil", evil),
        Player::with_name("Tit for tat", tit_for_tat),
        Player::with_name("Tit for two tats", tit_for_two_tats),
        Player::with_name("Greedy and friendly", greedy_blue_and_friendly),
        Player::with_name("Greedy and evil", greedy_blue_and_evil),
    ];

    let scores = init_scores(players.len());
    let rounds = random_rounds(MIN_ROUNDS, MAX_ROUNDS);

    println!("Pairing every program... ({0} games)\n", players.len() * (players.len() + 1) / 2);

    for i in 0..players.len() {
        for j in 0..players.len() {
            if i > j {
                continue;
            }

            add_game(
                players[i].clone(),
                players[j].clone(),
                &pool,
                rounds,
                (scores[i].clone(), scores[j].clone()),
            );
        }
    }

    pool.join();

    let mut scores = scores.iter()
        .enumerate()
        .map(|(i, v)| (*v.lock().unwrap(), players[i].get_name()))
        .collect::<Vec<_>>();
    scores.sort();

    println!("{rounds} rounds!");
    for (i, v) in scores.iter().rev().enumerate() {
        println!("{0}. {2} - {1:.2}", i + 1, v.0 as f32 / players.len() as f32, v.1);
    } 

    println!("\nProgram end");
}

/// Uniformly randomly select an amount of rounds between `min` and `max` (inclusive).
/// 
/// # Arguments
/// 
/// * `min` - The minimum amount of rounds
/// * `max` - The maximum amount of rounds
/// 
/// # Panics
/// 
/// If `max < min` or if `min == 0`.
fn random_rounds(min: u32, max: u32) -> u32 {
    if max < min {
        panic!("max_rounds cannot be less than min_rounds");
    }

    if min == 0 {
        panic!("min_rounds cannot not be 0");
    }

    distributions::Uniform::from(min..max+1).sample(&mut rand::thread_rng())
}

/// Create a vector of atomically reference counted and mutable score counters.
/// 
/// # Arguments
/// 
/// * `length` - The length of the vector to be created (typically `length` is equal to the amount of players)
/// 
/// # Panics
/// 
/// If `length == 0`.
fn init_scores(length: usize) -> Vec<Arc<Mutex<i32>>> {
    if length == 0 {
        panic!("Cannot initialize with a length of 0");
    }

    let mut scores = Vec::new();
    for _ in 0..length {
        scores.push(Arc::new(Mutex::new(0)));
    }
    
    scores
}

/// Add a game as a job to the provided threadpool `pool`.
/// 
/// # Arguments
/// 
/// * `player_1`- A [player](Player)
/// * `player_2`- A [player](Player) (may be the same as `player_1`)
/// * `pool` - The threadpool to execute the game
/// * `rounds` - The amount of rounds to play the game
/// * `score_totals` - Total score values for the players in the tournament
fn add_game(player_1: Player, player_2: Player, pool: &ThreadPool, rounds: u32, score_totals: (Arc<Mutex<i32>>, Arc<Mutex<i32>>)) {
    pool.execute(move || {
        let name = format!("{0:>20}  vs.  {1:<20}", player_1.get_name(), player_2.get_name());
        let scores = play(player_1, player_2, rounds);

        let output = format!("{0}   {1:>3} - {2:<3}", name, scores.0, scores.1);
        println!("{output}\n");

        if let Ok(mut score) = score_totals.0.lock() {
            *score += scores.0;
        }

        if let Ok(mut score) = score_totals.1.lock() {
            *score += scores.1;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_game_test() {
        let pool = ThreadPool::with_name("Games".into(), 20);

        let players = vec![
            Player::with_name("1", greedy_blue_and_evil),
            Player::with_name("2", take_back),
        ];

        let scores = init_scores(players.len());

        add_game(
            players[0].clone(),
            players[1].clone(),
            &pool,
            10,
            (scores[0].clone(), scores[1].clone())
        );

        add_game(
            players[0].clone(),
            players[1].clone(),
            &pool,
            0,
            (scores[0].clone(), scores[1].clone())
        );

        pool.join();

        let mut scores = scores.iter()
            .enumerate()
            .map(|(i, v)| (*v.lock().unwrap(), players[i].get_name()))
            .collect::<Vec<_>>();
        scores.sort();

        assert_eq!(scores, [(5, "2"), (36, "1")]);
    }

    #[test]
    #[should_panic(expected = "length")]
    fn init_scores_test() {
        init_scores(0);
    }

    #[test]
    #[should_panic(expected = "less than")]
    fn random_rounds_test_0() {
        random_rounds(10, 1);
    }

    #[test]
    #[should_panic(expected = "be 0")]
    fn random_rounds_test_1() {
        random_rounds(0, 0);
    }

    #[test]
    fn random_rounds_test_2() {
        assert!(random_rounds(10, 10) == 10);
        assert!(random_rounds(10, 11) < 12);
    }
}