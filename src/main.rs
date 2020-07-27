mod config;
use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, Error, LineWriter};
use std::path::Path;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

fn executable_path() -> Result<String, Error> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("config.json");
    Ok(dir.display().to_string())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = config::Config::parse(executable_path()?)?;
    let workers = config.files.len();
    println!("File merger started with {} workers...", workers);

    let pool = ThreadPool::with_name(String::from("laser_map"), workers);
    let (sending_channel, receiving_channel) = channel();
    for path in config.files {
        let sender = sending_channel.clone();
        let directory = Path::new(&config.directory).join(path);
        pool.execute(move || {
            if let Ok(lines) = read_lines(directory) {
                for line in lines {
                    if let Ok(data) = line {
                        if let Err(error) = sender.send(data) {
                            eprintln!("{:?}", error);
                        }
                    }
                }
            }
        });
    }
    pool.join();
    drop(sending_channel);

    let file = File::create(config.out_file)?;
    let mut file = LineWriter::new(file);

    while let Ok(data) = receiving_channel.recv() {
        file.write_all(data.as_bytes())?;
    }
    file.flush()?;
    Ok(())
}
