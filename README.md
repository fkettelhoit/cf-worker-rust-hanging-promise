# CF Worker Hanging Promise

**As of 2021/08/06 this issue has been fixed, see [this post](https://community.cloudflare.com/t/a-hanging-promise-was-canceled-rust-wasm-worker/282471/8) for more information. The test in this repository now passes, even when the calls to the Rust worker are made in parallel.**

This is a minimal test case for reproducing a hanging promise in a Cloudflare Worker. Sending multiple concurrent requests to the Rust Cloudflare Worker of this repo will lead to the following error:

```
HTTP/1.1 500 Internal Server Error
A hanging Promise was canceled. This happens when the worker runtime is waiting for a Promise from JavaScript to resolve, but has detected that the Promise cannot possibly ever resolve because all code and events related to the Promise's request context have already finished.
Uncaught (in response)
Error: The script will never generate a response.
```

The error originally occurred while awaiting a Workers KV operation, but it seems to be reproducible with any JS promise that is awaited in Rust.

## How to Reproduce

Change the `account_id` in `wrangler.toml`, then start the worker locally using `wrangler dev` or publish it to CF using `wrangler publish`. There are tests that call the worker in `tests/client.rs`, which should pass when run sequentially (using `cargo test sequential` for the published worker or `cargo test sequential --features workers-localhost` if the worker was started using `wrangler dev`), but will fail non-deterministically when run in parallel (using `cargo test parallel` for the published worker or `cargo test parallel --features workers-localhost` in the case of `wrangler dev`).
