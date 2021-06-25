# CF Worker Hanging Promise

This is a minimal test case for reproducing a hanging promise in a Cloudflare Worker. Sending multiple concurrent requests to the Rust Cloudflare Worker of this repo will lead to the following error:

```
HTTP/1.1 500 Internal Server Error
A hanging Promise was canceled. This happens when the worker runtime is waiting for a Promise from JavaScript to resolve, but has detected that the Promise cannot possibly ever resolve because all code and events related to the Promise's request context have already finished.
Uncaught (in response)
Error: The script will never generate a response.
```

The error originally occurred while awaiting a Workers KV operation, but it seems to be reproducible with any JS promise that is awaited in Rust.

## How to Reproduce

Change the `account_id` in `wrangler.toml`, then start the worker locally using `wrangler dev` or publish it to CF using `wrangler publish`. There are tests that call the worker in `tests/client.rs`, which should pass when run sequentially (using `cargo test -- --test-threads=1` for the published worker or `cargo test --features workers-localhost -- --test-threads=1` if the worker was started using `wrangler dev`), but will fail non-deterministically when run in parallel (using `cargo test` for the published worker or `cargo test --features workers-localhost` in the case of `wrangler dev`).
