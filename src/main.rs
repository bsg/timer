use std::{process::Command, thread, time::Duration};

use clap::Parser;

#[derive(Parser)]
#[command()]
struct Args {
    duration: String,

    #[arg(short, long)]
    repeat: bool,

    command: String,
    args: Vec<String>,
}
fn main() {
    let args = Args::parse();
    let duration = parse_duration::parse(&args.duration).unwrap();

    let mut remaining = duration;
    loop {
        if remaining.is_zero() {
            Command::new(&args.command)
                .args(&args.args)
                .spawn()
                .unwrap();
            match args.repeat {
                true => remaining = duration,
                false => break,
            }
        };

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!(
            "{:02}:{:02}:{:02}",
            (remaining.as_secs() / 60) / 60,
            (remaining.as_secs() / 60) % 60,
            remaining.as_secs() % 60
        );
        thread::sleep(Duration::from_millis(200));
        remaining -= Duration::from_millis(200);
    }
}
