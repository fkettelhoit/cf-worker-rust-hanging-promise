//! Test suite for calling the worker from a client

#![cfg(not(target_arch = "wasm32"))]

use futures::future::try_join_all;

// Depends on running `wrangler dev` to start a worker on localhost.

// The tests will (usually) fail if run in parallel using `cargo test parallel`
// but always pass if run using `cargo test sequential`, as the async promise
// that Rust is waiting on is apparently canceled if too many requests are handled in
// parallel.

const BASE_URL: &str = if cfg!(feature = "workers-localhost") {
    "http://localhost:8787"
} else {
    "https://cf-worker-rust-hanging-promise.assemblage.workers.dev"
};

const NUM_REQUESTS: u32 = 100;

#[tokio::test]
async fn parallel_test() -> Result<(), reqwest::Error> {
    let mut requests = Vec::new();
    for _ in 0..NUM_REQUESTS {
        requests.push(reqwest::get(BASE_URL));
    }
    let responses = try_join_all(requests).await?;
    for resp in responses.into_iter() {
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
    }
    Ok(())
}

#[test]
fn sequential_test() -> Result<(), reqwest::Error> {
    let mut responses = Vec::new();
    for _ in 0..NUM_REQUESTS {
        responses.push(reqwest::blocking::get(BASE_URL)?);
    }
    for resp in responses.into_iter() {
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
    }
    Ok(())
}
