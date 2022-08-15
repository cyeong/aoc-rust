use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub struct Config {
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Insufficient param, pass in filename");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut horizontal = 0;
    let mut depth = 0;

    let lines = read_lines(config.file_path)?;
    for line in lines {
        if let Ok(input) = line {
            let tokens: Vec<&str> = input.split(" ").collect();
            match tokens[0] {
                "forward" => horizontal += tokens[1].to_string().parse::<i32>().unwrap(),
                "up" => depth -= tokens[1].to_string().parse::<i32>().unwrap(),
                "down" => depth += tokens[1].to_string().parse::<i32>().unwrap(),
                _ => println!("Unexpected: {}", tokens[0]),
            }
        }
    }

    let total = horizontal * depth;
    println!("Final: {total}");
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
