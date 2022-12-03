use clap::Parser;
use dotenv;
use tokio;

mod aoc;
mod solutions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    day: u8,

    #[arg(long)]
    adv: bool,

    #[arg(long)]
    solve: bool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let aoc_session = std::env::var("AOC_SESSION").unwrap_or(String::new());

    let args = Args::parse();
    let mut result: Option<String> = None;
    if args.solve {
        let data = aoc::datafiles::load_data(args.day, aoc_session)
            .await
            .unwrap();
        result = Some(data);
    }

    let solution = match args.day {
        1 => solutions::day01::solve(result, !args.adv),
        2 => solutions::day02::solve(result, !args.adv),
        _ => format!("none, invalide day {}", args.day),
    };

    println!("Solution: {}", solution);
}
