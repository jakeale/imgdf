mod cli;
mod dhash;

use std::{sync::mpsc, thread, time::Duration};

use cli::parse_args;
use colored::Colorize;
use dhash::Dhash;

fn run() -> anyhow::Result<()> {
    let images = parse_args()?;

    let distance = Dhash::new(&images[0])?.hamming_distance(Dhash::new(&images[1])?)?;

    let similarity = ((64.0 - (distance as f32)) / 64.0) * 100.0;

    println!("{} {}", "Hamming distance:".blue().bold(), distance);
    println!("{} {}%", "Similarity: ".blue().bold(), similarity);

    Ok(())
}

fn do_some_threading() {
    let (sender, receiver) = mpsc::channel();

    let threads = 5;

    for _ in 0..threads {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            sender_clone
                .send(thread::current().id())
                .expect("thread id");
            thread::sleep(Duration::from_secs(1));
        });
    }

    let mut thread_ids = Vec::new();
    let mut received_cnt = 0;
    for received in receiver {
        println!("pushing {:?}", received);
        thread_ids.push(received);

        received_cnt += 1;
        if received_cnt == threads {
            break;
        }
    }
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} {:#}", "Error:".red(), e);
            std::process::exit(1);
        }
    }
}
