use std::cmp;
use std::process::{Command, exit};
use std::{thread, time};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = u32::MAX)]
    tries: u32,
    #[clap(short, long = "delay", value_parser, default_value_t = 0)]
    delay_s: u64,
    #[clap(short, long = "max-delay", value_parser, default_value_t = u64::MAX)]
    max_delay_s: u64,
    #[clap(short, long, value_parser, default_value_t = 1)]
    backoff: u64,
    #[clap(short, long, value_parser, default_value_t = false)]
    quiet: bool,
    #[clap(value_parser)]
    utility: String,
    #[clap(value_parser)]
    argument: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut next_delay_s = args.delay_s;
    let mut count = 0u32;
    loop {
        let mut command = Command::new(&args.utility);
        for a in &args.argument {
            command.arg(a);
        }

        match command.status() {
            Ok(status) => {
                if status.success() {
                    exit(0);
                }
            },
            Err(e) => {
                if !args.quiet {
                    eprintln!("Error: Could not spawn child process. {}", e);
                }
                exit(1);
            },
        }

        if count == args.tries {
            if !args.quiet {
                eprintln!("Error: Maximum number of retries reached");
            }
            exit(1);
        }
        count += 1;

        if !args.quiet {
            println!("Retrying({}): Wait {} seconds", count, next_delay_s);
        }
        thread::sleep(time::Duration::from_secs(next_delay_s));

        next_delay_s *= args.backoff;
        next_delay_s = cmp::min(next_delay_s, args.max_delay_s);
    }
}
