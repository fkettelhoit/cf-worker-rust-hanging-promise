//! Test suite for calling the worker from a client

#![cfg(not(target_arch = "wasm32"))]

// Depends on running `wrangler dev` to start a worker on localhost.

// The tests will (usually) fail if run in parallel using `cargo test` but
// always pass if run using `cargo test -- --test-threads=1`, as the async
// promise that Rust is waiting on is canceled on the next request.

const BASE_URL: &str = if cfg!(feature = "workers-localhost") {
    "http://localhost:8787"
} else {
    "https://cf-worker-rust-hanging-promise.assemblage.workers.dev"
};

fn run_test() -> Result<(), reqwest::Error> {
    let resp = reqwest::blocking::get(BASE_URL)?;
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
    Ok(())
}

#[test]
fn test1() -> Result<(), reqwest::Error> {
    run_test()
}

#[test]
fn test2() -> Result<(), reqwest::Error> {
    run_test()
}

#[test]
fn test3() -> Result<(), reqwest::Error> {
    run_test()
}

#[test]
fn test4() -> Result<(), reqwest::Error> {
    run_test()
}

#[test]
fn test5() -> Result<(), reqwest::Error> {
    run_test()
}

#[test]
fn test6() -> Result<(), reqwest::Error> {
    run_test()
}

#[test]
fn test7() -> Result<(), reqwest::Error> {
    run_test()
}

#[test]
fn test8() -> Result<(), reqwest::Error> {
    run_test()
}

#[test]
fn test9() -> Result<(), reqwest::Error> {
    run_test()
}
