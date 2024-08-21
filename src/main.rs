use log::info;
use std::{env, time::Duration, time::Instant};
use tokio;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    env_logger::init();
    let start = Instant::now();
    info!("Starting the Web Crawler!");
    let mut handles = vec![];
    for arg in env::args().skip(1) {
        handles.push(tokio::spawn(fetch(arg)));
    }
    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    let duration = start.elapsed();
    info!("Web Crawler has finished in: {:?}!", duration);
}

async fn fetch(url: String) {
    info!("fetching {}", url);
    let start = Instant::now();
    tokio::time::sleep(Duration::from_secs(2)).await;
    let duration = start.elapsed();
    info!("{} fetched in {:?}", url, duration);
}
