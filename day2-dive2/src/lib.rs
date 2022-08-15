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

#[derive(Debug)]
struct Submarine {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }

    fn dive(&mut self, tokens: Vec<&str>) {
        let action = tokens[0];
        let num = tokens[1].to_string().parse::<i32>().unwrap();
        match action {
            "forward" => self.proceed(num),
            "up" => self.aim -= num,
            "down" => self.aim += num,
            _ => println!("Unexpected: {}", tokens[0]),
        }
    }

    fn proceed(&mut self, num: i32) {
        self.horizontal += num;
        self.depth += self.aim * num;
    }

    fn location(&self) {
        let total = self.horizontal * self.depth;
        println!("We're at {total}")
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let lines = read_lines(config.file_path)?;

    let mut sub = Submarine::new();
    for line in lines {
        if let Ok(input) = line {
            let tokens: Vec<&str> = input.split(" ").collect();
            sub.dive(tokens)
        }
    }

    sub.location();

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
