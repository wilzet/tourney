//! `config` contains logic for configuring and running the tournament

use std::{sync::{Arc, Mutex}, cmp::Ordering};
use rand::{prelude::*, distributions};
use threadpool::ThreadPool;
use constcat::concat;
use crate::game::{Player, play};

/// The default value for the minimum amount of rounds
pub const MIN_ROUNDS: u32 = 70;
/// The default value for the maximum amount of rounds
pub const MAX_ROUNDS: u32 = 100;

const DEFAULT_THREADS: usize = 20;
const MAX_THREADS: usize = 64;
const MAX_THREADS_STRING: &str = "64";

/// Holds configurations for the tournament
#[derive(Debug)]
pub struct Config {
    rounds: u32,
    show_games: bool,
    threadpool: ThreadPool,
}

impl Config {
    /// Parse command line arguments
    fn parse_args(args: &[String]) -> Result<Config, &'static str> {
        // Command line arguments
        let mut min = 0;        // --min <u32>
        let mut max = 0;        // --max <u32>
        let mut games = false; // --games
        let mut threads = 0;  // --threads <u32>

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--min" => {
                    if min == 0 {
                        if let Some(value) = args.iter().skip(i + 1).next().and_then(|s| s.parse().ok()) {
                            if value > 0 {
                                min = value;
                                i += 2;
                                continue;
                            }

                            return Err("Value must be greater than 0 for argument: --min");
                        }

                        return Err("Incorrect value for argument: --min");
                    }

                    return Err("Duplicate argument: --min");
                }
                "--max" => {
                    if max == 0 {
                        if let Some(value) = args.iter().skip(i + 1).next().and_then(|s| s.parse().ok()) {
                            if value > 0 {
                                max = value;
                                i += 2;
                                continue;
                            }

                            return Err("Value must be greater than 0 for argument: --max");
                        }

                        return Err("Incorrect value for argument: --max");
                    }

                    return Err("Duplicate argument: --max");
                }
                "--games" => {
                    if !games {
                        games = true;
                        i += 1;
                        continue;
                    }

                    return Err("Duplicate argument: --games");
                }
                "--threads" => {
                    if threads == 0 {
                        if let Some(value) = args.iter().skip(i + 1).next().and_then(|s| s.parse().ok()) {
                            if value > 0 && value <= MAX_THREADS {
                                threads = value;
                                i += 2;
                                continue;
                            }

                            if value > MAX_THREADS {
                                return Err(concat!("Value must be less than or equal to ", MAX_THREADS_STRING, " for argument: --threads"));
                            }

                            return Err("Value must be greater than 0 for argument: --threads");
                        }

                        return Err("Incorrect value for argument: --threads");
                    }

                    return Err("Duplicate argument: --threads");
                }
                _ => {
                    return Err("Invalid arguments");
                }
            };
        }

        if min == 0 && max == 0 {
            min = MIN_ROUNDS;
            max = MAX_ROUNDS;
        } else if min == 0 {
            min = max;
        } else if max == 0 {
            max = min;
        }

        if threads == 0 {
            threads = DEFAULT_THREADS;
        }

        Ok(Config {
            rounds: random_rounds(min, max),
            show_games: games,
            threadpool: ThreadPool::with_name("Games".into(), threads),
        })
    }

    /// Try to create a new `Config` from command line arguments.
    /// 
    /// # Arguments
    /// 
    /// * `args` - Command line arguments
    /// 
    /// ## Commands
    /// 
    /// * `--min <u32>` - The minimum amount of rounds
    /// * `--max <u32>` - The maximum amount of rounds
    /// * `--games` - Displays all the games outcomes if this is provided
    /// * `--threads <u32>` - Specify the amount of threads used
    /// 
    /// If only `--min` is provided, the config will have `rounds == --min`.
    /// Likewise if only `--max` is provided, the config will have `rounds == --max`.
    /// 
    /// Duplicates are not allowed.
    /// 
    /// # Returns
    /// 
    /// A config if all commands can be parsed.
    /// 
    /// # Errors
    /// 
    /// If the provided `args` cannot be parsed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tourney::config::*;
    /// 
    /// let config = Config::new(&[String::from("tourney"), String::from("--min"), String::from("10")]).unwrap();
    /// let error = Config::new(&[String::from("tourney"), String::from("--threads"), String::from("6"), String::from("--threads")]).expect_err("");
    /// 
    /// assert!(config.rounds() == 10);
    /// assert_eq!(error, "Duplicate argument: --threads");
    /// ```
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Ok(Config::default());
        }

        Config::parse_args(&args)
    }

    /// Create a new `Config` with default values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use tourney::config::*;
    /// let config = Config::default();
    /// ```
    pub fn default() -> Config {
        Config {
            rounds: random_rounds(MIN_ROUNDS, MAX_ROUNDS),
            show_games: false,
            threadpool: ThreadPool::with_name("Games".into(), DEFAULT_THREADS),
        }
    }

    /// Get the configured amount of rounds.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use tourney::config::*;
    /// let config = Config::default();
    /// 
    /// assert!(config.rounds() <= MAX_ROUNDS)
    /// ```
    pub fn rounds(&self) -> u32 {
        self.rounds
    }

    /// Add a game to the config threadpool.
    /// 
    /// # Arguments
    /// 
    /// * `player_1`- A [player](Player)
    /// * `player_2`- A [player](Player) (may be the same as `player_1`)
    /// * `score_totals` - Total score values for the players in the tournament
    fn add_game(&self, player_1: Player, player_2: Player, score_totals: (Arc<Mutex<(i32, i32)>>, Arc<Mutex<(i32, i32)>>)) {
        let rounds = self.rounds;
        let show_games = self.show_games;
        self.threadpool.execute(move || {
            let name = format!("{0:>20}  vs.  {1:<20}", player_1.get_name(), player_2.get_name());
            let scores = play(player_1, player_2, rounds);

            if show_games {
                let output = format!("{0}   {1:>3} - {2:<3}", name, scores.0, scores.1);
                println!("{output}\n");
            }
    
            if let Ok(mut score) = score_totals.0.lock() {
                score.0 += scores.0;
                score.1 += match scores.0.cmp(&scores.1) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                }
            }
    
            if let Ok(mut score) = score_totals.1.lock() {
                score.0 += scores.1;
                score.1 += match scores.1.cmp(&scores.0) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                }
            }
        });
    }
}

/// Create a vector of atomically reference counted and mutable score and relative win counters.
/// 
/// # Arguments
/// 
/// * `length` - The length of the vector to be created (typically `length` is equal to the amount of players)
/// 
/// # Panics
/// 
/// If `length == 0`.
fn init_scores(length: usize) -> Vec<Arc<Mutex<(i32, i32)>>> {
    if length == 0 {
        panic!("Cannot initialize scores with a length of 0");
    }

    let mut scores = Vec::new();
    for _ in 0..length {
        scores.push(Arc::new(Mutex::new((0, 0))));
    }
    
    scores
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
        panic!("max rounds cannot be less than min rounds");
    }

    if min == 0 {
        panic!("min rounds cannot not be 0");
    }

    distributions::Uniform::from(min..max+1).sample(&mut rand::thread_rng())
}

/// Run a tournament of the [game](crate::game).
/// 
/// # Arguments
/// 
/// * `config` - A [config](Config) specifying rounds, output, etc.
/// * `players` - The [players](Player) for this tournament
/// 
/// # Returns
/// 
/// A list of the program names and associated scores sorted in descending order according to their scores.
/// 
/// # Errors
/// 
/// If `players.len() < 2` an error is returned.
/// 
/// # Examples
/// 
/// ```
/// # use tourney::programs::all::*;
/// use tourney::config::*;
/// use tourney::game::Player;
/// 
/// let config_1 = Config::new(&[String::from("tourney"), String::from("--min"), String::from("10")]).unwrap();
///
/// let players = vec![
///     Player::with_name("1", greedy_blue_and_evil),
///     Player::with_name("2", take_back_once_prisoner),
/// ];
///
/// let scores_1 = run(&config_1, &players).unwrap();
/// 
/// assert_eq!([(scores_1[0].0, scores_1[0].2), (scores_1[1].0, scores_1[1].2)], [(36, "1"), (5, "2")]);
/// ```
pub fn run<'a>(config: &Config, players: &'a Vec<Player>) -> Result<Vec<(i32, f32, &'a str)>, &'static str> {
    if players.len() < 2 {
        return Err("Too few players");
    }

    let player_count = players.len();

    let scores = init_scores(player_count);

    for i in 0..player_count {
        for j in 0..player_count {
            if i >= j {
                continue;
            }

            config.add_game(
                players[i].clone(),
                players[j].clone(),
                (scores[i].clone(), scores[j].clone()),
            );
        }
    }

    config.threadpool.join();

    let mut scores = scores.iter()
        .enumerate()
        .map(|(i, v)| {
            let v = v.lock().unwrap();
            (v.0, v.1 as f32 / (players.len() - 1) as f32, players[i].get_name())
        })
        .collect::<Vec<_>>();
    scores.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    Ok(scores.iter()
        .rev()
        .map(|v| *v)
        .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs::{
        prisoners::*,
        greedy::*,
    };

    #[test]
    fn config_test() {
        let config_1 = Config::new(&[String::from("tourney"), String::from("--min"), String::from("10")]).unwrap();
        let config_2 = Config::new(&[String::from("tourney"), String::from("--max"), String::from("5")]).unwrap();

        let players = vec![
            Player::with_name("1", greedy_blue_and_evil),
            Player::with_name("2", take_back_once_prisoner),
        ];

        let scores_1 = run(&config_1, &players).unwrap();
        let scores_2 = run(&config_2, &players).unwrap();

        assert_eq!([(scores_1[0].0, scores_1[0].2), (scores_1[1].0, scores_1[1].2)], [(36, "1"), (5, "2")]);
        assert_eq!([(scores_2[0].0, scores_2[0].2), (scores_2[1].0, scores_2[1].2)], [(14, "1"), (3, "2")]);
    }

    #[test]
    fn parsing_test() {
        let error = Config::new(&[String::from("tourney"), String::from("--threads"), String::from("6"), String::from("--threads")]).expect_err("parsing test");
        assert_eq!(error, "Duplicate argument: --threads");
    }

    #[test]
    #[should_panic(expected = "length")]
    fn init_scores_panic_test() {
        init_scores(0);
    }

    #[test]
    fn init_scores_test() {
        let scores = init_scores(10);
        assert_eq!(scores.len(), 10);
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