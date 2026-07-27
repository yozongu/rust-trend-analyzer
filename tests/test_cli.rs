use chrono::NaiveDate;
use clap::Parser;
use hamcrest2::prelude::*;
use rstest::rstest;
use semester_project::prelude::*;

#[rstest]
fn test_cli_struct() {
    let arguments = Args {
        keyword: "bitcoin".to_string(),
        begin_date: NaiveDate::from_ymd_opt(2026, 01, 01).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2026, 07, 01).unwrap(),
        compare_keyword: None,
    };
    assert_that!(arguments.keyword, equal_to("bitcoin"));
    assert_that!(arguments.begin_date.to_string(), equal_to("2026-01-01"));
    assert_that!(arguments.end_date.to_string(), equal_to("2026-07-01"));
}

#[rstest]
fn test_parse_cli_valid() {
    let arguments = Args::parse_from(["run", "bitcoin", "2025-01-01", "2026-01-01"]);
    assert_that!(arguments.keyword, equal_to("bitcoin"));
    assert_that!(arguments.begin_date.to_string(), equal_to("2025-01-01"));
    assert_that!(arguments.end_date.to_string(), equal_to("2026-01-01"));
}

#[rstest]
fn test_parse_cli_invalid() {
    let result = Args::try_parse_from(["United", "States", "News"]);
    assert_that!(result.is_err(), equal_to(true));
}

#[rstest]
fn test_parse_cli_with_optional_flag() {
    let arguments = Args::parse_from(["run", "bitcoin", "2025-01-01", "2026-01-01", "--compare", "etherium"]);
    assert_that!(arguments.keyword, equal_to("bitcoin"));
    assert_that!(arguments.begin_date.to_string(), equal_to("2025-01-01"));
    assert_that!(arguments.end_date.to_string(), equal_to("2026-01-01"));
    assert_that!(arguments.compare_keyword, equal_to(Some("etherium".to_string())));
}
