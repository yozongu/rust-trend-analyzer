# CS395 Semester Project - Rust Trend Analyzer

## Requirements

- Rust installed: https://www.rust-lang.org/tools/install
- Terminal or IDE
- Working python install with matplotlib library (pip install matplotlib)
- Python needed be added to PATH

## Dependencies

- aletheia = "1.1.0"
- chrono = "0.4.45"
- clap = { version = "4.6.4", features = ["derive"] }
- csv = "1.4.0"
- dotenvy = "0.15.7"
- pyo3 = { version = "0.29", features = ["auto-initialize"] }
- reqwest = { version = "0.13.4", features = ["json"] }
- tokio = { version = "1.53.1", features = ["full"] }

## .env File

- Create a .env file in project root with the format:  
  GUARDIAN_API_KEY=your_api_key_here
- To get a Guardian API key visit: https://open-platform.theguardian.com/access/
- NOTE: The Guardian API allows 500 calls/day and 1 call/second for free. Avoid using broad range larger than 2 years.

## Compilation

- Use cargo build

## Run

- To run use the format: cargo run "key word" YYYY-MM-DD YYYY-MM-DD
- First date is begin date, second is end date

## Sample Execution & Output
- If the input contains: cargo run "world cup" 2025-01-01 2026-07-24
- The output will include a CSV file with the format: date,count

```
date,count
2026-01-01,71
2026-01-02,65
2026-01-03,54
2026-01-04,61
2026-01-05,89
2026-01-06,89
2026-01-07,79
2026-01-08,80
2026-01-09,101
...
```
- A graph called "trend.png" will be generated showing trend of key word over specified input time:
  ![Trend analysis for "world cup" from January 01, 2026 to July 24, 2026](/trend.png)
