addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

// instantiate these once, and await the Promise returned by
// `wasm_bindgen(wasm)` inside the request handler.
const { handle } = wasm_bindgen;
const instance =  wasm_bindgen(wasm);

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    await instance;

    const objectWithAsyncMethod = {
      run: () => new Promise(resolve => setTimeout(resolve, 10))
    };

    const greeting = await handle(objectWithAsyncMethod);
    return new Response(greeting, {status: 200});
}
