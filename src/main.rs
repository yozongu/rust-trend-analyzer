mod parser;
use parser::*;
use semester_project::cli::{parse_cli};
use semester_project::graph_builder::plot_data_from_csv;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let args = parse_cli();
    let keyword = &args.keyword.to_ascii_lowercase();
    let begin_date = &args.begin_date;
    let end_date = &args.end_date;

    let content = match get_content(&keyword, &begin_date,&end_date).await {
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
    plot_data_from_csv(&csv_filename, &keyword).unwrap();
    // println!("{:#?}", parse_cli());
    
}
