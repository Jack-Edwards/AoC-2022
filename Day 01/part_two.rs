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

    let mut elf_calories: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(config.file_path) {

        let mut current_elf_calories: i32 = 0;
        for line in lines {
            if let Ok(value) = line {
                if value.is_empty() {
                    elf_calories.push(current_elf_calories.clone());
                    current_elf_calories = 0;
                } else {
                    current_elf_calories += value.parse::<i32>().unwrap();
                }
            }
        }
    }

    let sorted_elf_calories = sort_vector(elf_calories);
    let total_calories_top_three_elves: i32 = sorted_elf_calories[0..3].iter().sum();

    println!("The three elves carrying the most calories are carrying a total of {} calories", total_calories_top_three_elves);
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

fn sort_vector(mut vector: Vec<i32>) -> Vec<i32> {
    for i in 0..vector.len() {
        let mut j = i + 1;
        while j < vector.len() {
            if vector[i] < vector[j] {
                let temp = vector[i];
                vector[i] = vector[j];
                vector[j] = temp;
            }
            j += 1;
        }
    }
    return vector;
}
