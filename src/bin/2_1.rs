use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn main() {
    let player_move_scores: HashMap<char, i32> = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut total_player_score = 0;

    if let Ok(lines) = read_lines(config.file_path) {
        for line in lines {
            if let Ok(value) = line {
                if !value.is_empty() {
                    let mut characters = value.chars();

                    let opponent_move = characters.next();
                    
                    characters.next();

                    let player_move = characters.next();
                    total_player_score += player_move_scores[&player_move.unwrap()];

                    total_player_score += get_score(player_move.unwrap(), opponent_move.unwrap());
                }
            }
        }
    }

    println!("Your total score would be: {}", total_player_score);
}

struct Config {
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Input path not provided.")
        }

        let file_exists = Path::new(&args[1]).exists();
        if !file_exists {
            return Err("Input path does not exist.");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

fn read_lines<T>(filepath: T) -> io::Result<io::Lines<io::BufReader<File>>>
where T: AsRef<Path>, {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_score(player_move: char, opponent_move: char) -> i32 {
    let player_lost = 0;
    let draw = 3;
    let player_win = 6;

    let opponent_rock = 'A';
    let opponent_paper = 'B';
    let opponent_scissors = 'C';

    let player_rock = 'X';
    let player_paper = 'Y';
    let player_scissors = 'Z';

    if player_move == player_rock {
        if opponent_move == opponent_rock {
            return draw;
        } else if opponent_move == opponent_paper {
            return player_lost;
        } else if opponent_move == opponent_scissors {
            return player_win;
        }
    } else if player_move == player_paper {
        if opponent_move == opponent_rock {
            return player_win;
        } else if opponent_move == opponent_paper {
            return draw;
        } else if opponent_move == opponent_scissors {
            return player_lost;
        }
    } else if player_move == player_scissors {
        if opponent_move == opponent_rock {
            return player_lost;
        } else if opponent_move == opponent_paper {
            return player_win;
        } else if opponent_move == opponent_scissors {
            return draw;
        }
    }
    return 0;
}