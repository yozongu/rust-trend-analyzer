use roux::Reddit;
use std::env;

pub fn intialize_account() {
    dotenvy::dotenv().ok();
    let REDDIT_CLIENT_ID = env::Var("REDDIT_CLIENT_ID");
    let REDDIT_CLIENT_SECRET = env::var("REDDIT_CLIENT_SECRET");
    let REDDIT_USERNAME = env::var("REDDIT_USERNAME");
    let REDDIT_PASSWORD = env::var("REDDIT_PASSWORD");

    let client = Reddit::new("rust:reddit-scraper:v1.0.0 (by /u/DatabaseTemporary253)", REDDIT_CLIENT_ID, REDDIT_CLIENT_SECRET)
        .username(REDDIT_USERNAME)
        .password(REDDIT_PASSWORD)
        .login()
        .await;
    let me = client.unwrap();
}
