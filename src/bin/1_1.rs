use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut elf_carrying_most_calories: i32 = 0;
    let mut most_calories: i32 = 0;

    let mut current_elf: i32 = 0;
    let mut current_elf_calories: i32 = 0;

    if let Ok(lines) = read_lines(config.file_path) {
        for line in lines {
            if let Ok(value) = line {
                if value.is_empty() {
                    current_elf += 1;

                    if current_elf_calories > most_calories {
                        elf_carrying_most_calories = current_elf;
                        most_calories = current_elf_calories;
                    }

                    current_elf_calories = 0;
                } else {
                    current_elf_calories += value.parse::<i32>().unwrap();
                }
            }
        }
    }

    println!("Elf # {} is carrying the most calories: {}", elf_carrying_most_calories, most_calories);
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
