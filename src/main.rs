mod parser;
use parser::*;
use semester_project::graph_builder::export_daily_counts_to_csv;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let keyword = "bitcoin";
    let content = match get_content(&keyword, (2026,1,1),(2026,7,22)).await {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error fetching content: {e}");
            return;
        }
    };
    let processed = aggregate_by_day(&content);
    for (date, count) in &processed {
        println!("{date}: {count} articles");
    }
    let csv_filename = match export_daily_counts_to_csv(&processed) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error writing to csv file: {e}");
            return;
        }
    };
}
