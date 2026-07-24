use aletheia::{GuardianContentClient, enums::*, structs::SearchResult};
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;
use std::env;

pub async fn get_content(
    keyword: &str,
    date_from: &NaiveDate,
    date_to: &NaiveDate,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let guardian_api_key =
        env::var("GUARDIAN_API_KEY").expect("GUARDIAN_API_KEY must be set in .env");

    let client = GuardianContentClient::new(&guardian_api_key);

    fetch_all_pages(&client, keyword, date_from, date_to).await
}

async fn fetch_all_pages(
    client: &GuardianContentClient,
    keyword: &str,
    date_from: &NaiveDate,
    date_to: &NaiveDate,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let mut all_results = Vec::new();
    let mut current_page: u32 = 1;

    loop {
        let page_results = fetch_page(client, keyword, date_from, date_to, current_page).await?;
        let got_count = page_results.len();
        all_results.extend(page_results);

        if got_count < 200 {
            break;
        }
        current_page += 1;
    }

    Ok(all_results)
}

async fn fetch_page(
    client: &GuardianContentClient,
    keyword: &str,
    date_from: &NaiveDate,
    date_to: &NaiveDate,
    page: u32,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let response = client
        .build_request()
        .search(keyword)
        .date_from(date_from.year(), date_from.month(), date_from.day())
        .date_to(date_to.year(), date_to.month(), date_to.day())
        .page_size(200)
        .page(page)
        .show_fields(vec![Field::Byline, Field::LastModified])
        .order_by(OrderBy::Newest)
        .order_date(OrderDate::Published)
        .send()
        .await?;

    Ok(response.results.unwrap_or_default())
}

pub fn aggregate_by_day(results: &[SearchResult]) -> Vec<(NaiveDate, usize)> {
    let mut counts: HashMap<NaiveDate, usize> = HashMap::new();

    for result in results {
        if let Some(pub_date) = result.web_publication_date {
            let day = pub_date.date_naive();
            *counts.entry(day).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<(NaiveDate, usize)> = counts.into_iter().collect();
    sorted.sort_by_key(|(date, _)| *date);
    sorted
}
