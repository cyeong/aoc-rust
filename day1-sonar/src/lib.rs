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
    let mut current = -1;
    let mut increased_count = 0;

    let lines = read_lines(config.file_path)?;
    for line in lines {
        if let Ok(depth) = line {
            let depth_int = depth.to_string().parse::<i32>().unwrap();
            if current == -1 {
                current = depth_int;
            } else {
                if current < depth_int {
                    increased_count += 1;
                }
                current = depth_int;
            }
        }
    }
    println!("Increased: {increased_count}");
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
