mod core;

use chrono::{NaiveDateTime, Utc};
use clap::{Parser, Subcommand};
use core::{count_ticks, pretty_breakdown};

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
            let ticks = count_ticks(None, Some(now), &unit);
            println!("Ticks since epoch ({unit}): {ticks}");
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

            let output = pretty_breakdown(start_dt, end_dt, 3);

            println!("Between {} and {:?}: {}", start, end, output);
        }

        Commands::Until { target, unit } => {
            let now = Utc::now();
            let target_dt = NaiveDateTime::parse_from_str(&target, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc();

            if target_dt < now {
                println!("Target {} is in the past.", target);
            } else {
                let ticks = count_ticks(Some(now), Some(target_dt), &unit);
                println!("Until {}: {} {}", target, ticks, unit);
            }
        }


        Commands::Clock => {
            loop {
                let now = Utc::now();
                let secs = count_ticks(None, Some(now), "seconds");
                let millis = count_ticks(None, Some(now), "milliseconds");
                let micros = count_ticks(None, Some(now), "microseconds");

                println!("Since epoch: {secs}s | {millis}ms | {micros}µs");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
