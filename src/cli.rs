use chrono::NaiveDate;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub keyword: String,
    #[arg(value_parser = parse_date)]
    pub begin_date: NaiveDate,
    #[arg(value_parser = parse_date)]
    pub end_date: NaiveDate,
}

pub fn parse_cli() -> Args {
    let args = Args::parse();
    println!(
        "Searching for '{}' from {} to {}",
        args.keyword, args.begin_date, args.end_date
    );
    args
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("Please use the input format \"Cargo run \"key word\" YYYY-MM-DD YYYY-MM-DD\" where the first date is begin, and the second is end date"))
}
