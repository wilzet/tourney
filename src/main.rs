use threadpool::ThreadPool;
use rand::{prelude::*, distributions::Uniform};

mod game;
mod programs;

use game::{Player, play};
use programs::*;

const ROUNDS: u32 = 100;

fn main() {
    println!("Program start\n");
    let pool = ThreadPool::with_name("Games".into(), 20);

    let players = vec![
        Player::with_name_and_program("Take back 1".into(), take_back),
        Player::with_name_and_program("Friendly".into(), friendly),
        Player::with_name_and_program("Evil".into(), evil),
        Player::with_name_and_program("Tit for tat".into(), tit_for_tat),
        Player::with_name_and_program("Tit for two tats".into(), tit_for_two_tats)
    ];

    let range = Uniform::from(0..players.len());
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let i = range.sample(&mut rng);
        let j = range.sample(&mut rng);

        add_game(players[i].clone(), players[j].clone(), &pool);
    }

    pool.join();
    println!("\nProgram end");
}

fn add_game(player1: Player, player2: Player, pool: &ThreadPool, ) {
    pool.execute(move || {
        let name = format!("{0:>20}  vs.  {1:<20}", player1.get_name(), player2.get_name());
        let score = play(player1, player2, ROUNDS);

        let output = format!("{0}   {1:>3} - {2:<3}", name, score.0, score.1);
        println!("{output}\n");
    });
}