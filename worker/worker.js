addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { handle } = wasm_bindgen;
    await wasm_bindgen(wasm);

    const objectWithAsyncMethod = {
      run: () => new Promise(resolve => setTimeout(resolve, 10))
    };

    const greeting = await handle(objectWithAsyncMethod);
    return new Response(greeting, {status: 200});
}
