use chrono::{Local, NaiveDate};
use std::error::Error;

pub fn export_daily_counts_to_csv(
    data: &[(NaiveDate, usize)],
) -> Result<String, Box<dyn Error>> {
    let filename = generate_filename("Trend-data-", ".csv");

    let mut writer = csv::Writer::from_path(&filename)?;
    writer.write_record(["date", "count"])?;

    for (date, count) in data {
        writer.write_record([date.to_string(), count.to_string()])?;
    }

    writer.flush()?;
    Ok(filename)
}

pub fn generate_filename(prefix: &str, extension: &str) -> String {
    let now = Local::now();
    format!("{}_{}.{}", prefix, now.format("%Y-%m-%d_%H-%M-%S"), extension)
}
