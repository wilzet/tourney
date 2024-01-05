use threadpool::ThreadPool;
use rand::{prelude::*, distributions};

mod game;
mod programs;

use game::{Player, play};
use programs::*;

const MIN_ROUNDS: u32 = 90;
const MAX_ROUNDS: u32 = 100;

fn main() {
    if MIN_ROUNDS > MAX_ROUNDS {
        panic!("MIN_ROUNDS cannot be less than MAX_ROUNDS");
    }

    println!("Program start\n");
    let pool = ThreadPool::with_name("Games".into(), 20);

    let players = vec![
        Player::with_name_and_program("Take back 1".into(), take_back),
        Player::with_name_and_program("Friendly".into(), friendly),
        Player::with_name_and_program("Evil".into(), evil),
        Player::with_name_and_program("Tit for tat".into(), tit_for_tat),
        Player::with_name_and_program("Tit for two tats".into(), tit_for_two_tats),
        Player::with_name_and_program("Greedy and friendly".into(), greedy_blue_and_friendly),
        Player::with_name_and_program("Greedy and evil".into(), greedy_blue_and_evil),
    ];

    let rounds = distributions::Uniform::from(MIN_ROUNDS..MAX_ROUNDS+1).sample(&mut rand::thread_rng());

    println!("{rounds} rounds!");
    println!("Pairing every program... ({0} matches)\n", players.len() * (players.len() + 1) / 2);

    for i in 0..players.len() {
        for j in 0..players.len() {
            if i > j {
                continue;
            }

            add_game(players[i].clone(), players[j].clone(), &pool, rounds);
        }
    }

    pool.join();
    println!("\nProgram end");
}

fn add_game(player1: Player, player2: Player, pool: &ThreadPool, rounds: u32) {
    pool.execute(move || {
        let name = format!("{0:>20}  vs.  {1:<20}", player1.get_name(), player2.get_name());
        let score = play(player1, player2, rounds);

        let output = format!("{0}   {1:>3} - {2:<3}", name, score.0, score.1);
        println!("{output}\n");
    });
}