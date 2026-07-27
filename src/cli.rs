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
    #[arg(long = "compare")]
    pub compare_keyword: Option<String>
}

pub fn parse_cli() -> Args {
    let args = Args::parse();

    if args.compare_keyword.is_none() {
        println!(
            "Searching for '{}' from {} to {}",
            args.keyword, args.begin_date, args.end_date
        );
    }

    if let Some(compare_keyword) = &args.compare_keyword {
        println!(
            "Searching for '{}' from {} to {}, comparing with {}",
            args.keyword, args.begin_date, args.end_date, compare_keyword
        );
    }

    args
}

pub fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("Please use the input format \"Cargo run \"key word\" YYYY-MM-DD YYYY-MM-DD\" where the first date is begin, and the second is end date"))
}
