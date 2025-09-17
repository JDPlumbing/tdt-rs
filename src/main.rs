mod core;

use chrono::{NaiveDateTime, Utc};
use clap::{Parser, Subcommand};
use core::TimeDelta;

#[derive(Parser)]
#[command(name = "tdt")]
#[command(about = "Time Dimension Tool in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show ticks since epoch (or until a given date)
    Since {
        #[arg(short, long)]
        unit: String,
    },

    /// Show pretty breakdown between two dates
    Between {
        #[arg(short, long)]
        start: String,
        #[arg(short, long)]
        end: Option<String>,
    },

    /// Show ticks until a target future date
    Until {
        #[arg(short, long)]
        target: String,
        #[arg(short, long)]
        unit: String,
    },

    /// Live clock of ticks since epoch
    Clock,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Since { unit } => {
            let now = Utc::now();
            let epoch = NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc();

            let td = TimeDelta::between(epoch, now);
            println!("Ticks since epoch ({unit}): {}", td.ticks(&unit));
        }

        Commands::Between { start, end } => {
            let start_dt = NaiveDateTime::parse_from_str(&start, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc();

            let end_dt = end.as_ref().map(|s| {
                NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc()
            });

            let td = if let Some(end_dt) = end_dt {
                TimeDelta::between(start_dt, end_dt)
            } else {
                TimeDelta::until_now(start_dt)
            };

            println!("Between {} and {:?}: {}", start, end, td.pretty(3));
        }


        Commands::Until { target, unit } => {
            let now = Utc::now();
            let target_dt = NaiveDateTime::parse_from_str(&target, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc();

            if target_dt < now {
                println!("Target {} is in the past.", target);
            } else {
                let td = TimeDelta::between(now, target_dt);
                println!("Until {}: {} {}", target, td.ticks(&unit), unit);
            }
        }

        Commands::Clock => {
            let epoch = NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc();

            loop {
                let now = Utc::now();
                let td = TimeDelta::between(epoch, now);

                let secs = td.ticks("seconds");
                let millis = td.ticks("milliseconds");
                let micros = td.ticks("microseconds");

                println!("Since epoch: {secs}s | {millis}ms | {micros}µs");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
