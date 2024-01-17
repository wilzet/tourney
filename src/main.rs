use std::{env, process};
use tourney::config::*;
use tourney::programs::*;
use tourney::game::Player;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };

    println!("Program start\n");

    let players = vec![
        Player::with_name("Take back 1", take_back_once_prisoner),
        Player::with_name("Friendly", friendly),
        Player::with_name("Evil", evil),
        Player::with_name("Tit for tat", tit_for_tat_prisoner),
        Player::with_name("Tit for two tats", tit_for_two_tats_prisoner),
        Player::with_name("Greedy and friendly", greedy_blue_and_friendly),
        Player::with_name("Greedy and evil", greedy_blue_and_evil),
        Player::with_name("Greedy blue", blue),
        Player::with_name("Try to guess", try_to_guess),
        Player::with_name("Random", random),
        Player::with_name("ChatGPT adaptive", chat_gpt_adaptive),
        Player::with_name("ChatGPT proactive", chat_gpt_proactive),
        Player::with_name("ChatGPT versatile", chat_gpt_versatile),
        Player::with_name("Cooperate until defection", cooperate_until_defection),
        Player::with_name("Random, greedy if winning", greedy_if_winning_else_random),
        Player::with_name("Random, greedy if 2x", greedy_if_2x_score_else_random),
    ];

    println!("Pairing every program... ({0} games)\n", players.len() * (players.len() - 1) / 2);

    let scores = run(&config, &players).unwrap();

    println!("{0} rounds!", config.rounds());
    for (i, v) in scores.iter().enumerate() {
        println!("{0}. {2} - {1:.2}", i + 1, v.0 as f32 / players.len() as f32, v.1);
    }

    println!("\nProgram end");
}