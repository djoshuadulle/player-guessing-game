pub mod roster;

use crate::roster::Player;
use crate::roster::Solution;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{env, io, process};

enum GameState {
    Home,
    New,
    Active,
    Idle,
}

struct Config {
    file_path: String,
    num_options: usize,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // The first value in the return value of env::args is the name of the program, so ignore
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Input a file path."),
        };

        let num_options = match args.next() {
            Some(arg) => arg.parse::<usize>().unwrap(),
            None => return Err("Input the number of guess options."),
        };

        Ok(Config {
            file_path,
            num_options,
        })
    }
}

fn read_cli_input() -> String {
    let mut cli_input = String::new();

    // Idles waiting for input from command line
    io::stdin()
        .read_line(&mut cli_input)
        .expect("Reading from cursor won't fail...");

    cli_input = cli_input.replace('\n', "");
    cli_input
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Create a path to the roster csc file
    let path = PathBuf::from(config.file_path);
    // Open the path in read-only mode, returns `io::Result<File>`, then pass
    // to a BufReader to create an iterator over each line
    let roster_iter = match File::open(path.as_path()) {
        Ok(file) => BufReader::new(file).lines(),
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
    };

    let mut roster: Vec<Player> = Vec::new();

    for player_line in roster_iter {
        let player = match Player::parse_player_line(player_line.unwrap()) {
            Ok(player) => player,
            Err(why) => {
                eprintln!("Problem parsing roster: {why}");
                process::exit(1);
            }
        };
        roster.push(player);
    }

    // Validate inputs
    let roster_size = roster.len();
    if config.num_options > roster_size || config.num_options < 2 {
        panic!("Option count must be between 2 and {}.", roster_size);
    }

    println!("Welcome to the player guessing game...");

    let mut loopz: bool = true;
    let mut guess_idx: Option<usize> = None;
    let mut game_state: GameState = GameState::Home;
    let mut solution: Solution = Player::generate_new_player(&mut roster, config.num_options);

    // Main gameplay loop/state machine
    while loopz {
        match game_state {
            GameState::Home => {
                println!("Select: (n)ew, (q)uit");

                let cli_input: String = read_cli_input();

                // Check if input is game option: (q)uit, (n)ew
                if cli_input == "q" {
                    // Kill loop
                    loopz = false;
                } else if cli_input == "n" {
                    // Generate new player guess
                    game_state = GameState::New;
                } else {
                    println!("Please input valid option.");
                }
            }
            GameState::New => {
                solution = Player::generate_new_player(&mut roster, config.num_options);

                println!(
                    "\nWhich player wears number {}?",
                    solution.correct_player.number
                );
                for i in 0..config.num_options {
                    println!("{}) {}", i + 1, solution.option_list[i].name);
                }
                println!(" ");

                game_state = GameState::Idle;
            }
            GameState::Idle => {
                let cli_input: String = read_cli_input();

                guess_idx = match cli_input.parse::<u8>() {
                    Ok(idx) => {
                        let mut guess_idx: usize = idx.into();

                        if 0 < guess_idx && guess_idx <= config.num_options {
                            guess_idx -= 1;
                            game_state = GameState::Active;
                            Some(guess_idx)
                        } else {
                            println!("Guess must be valid option number.");
                            None
                        }
                    }
                    Err(_e) => {
                        println!("Input numeric value matching option number.");
                        None
                    }
                };
            }
            GameState::Active => {
                if let Some(guess_idx) = guess_idx {
                    if solution.option_list[guess_idx] == solution.correct_player {
                        println!(
                            "Correct! {} wears number {}\n",
                            solution.correct_player.name, solution.correct_player.number
                        );
                        game_state = GameState::Home;
                    } else {
                        println!("Nope!\n");
                        game_state = GameState::Idle;
                    }
                }
            }
        } // end match game_state
    } // close while loop
}
