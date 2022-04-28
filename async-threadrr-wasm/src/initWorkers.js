// Make sure workers stay in memory
let _workers;

export async function initWorkers(module, memory, no_blocking_amount, some_blocking_amount, much_blocking_amount) {
	if (no_blocking_amount < 0 || some_blocking_amount < 0 || much_blocking_amount < 0 || (no_blocking_amount + some_blocking_amount + much_blocking_amount) === 0) {
		throw new Error('Amount of workers must be > 0')
	}

	const wasmData = {
		module,
		memory
	}

	_workers = await Promise.all(
		Array.from({length: no_blocking_amount}, async () => {
			const worker = new Worker(new URL('./wasmWorker.js', import.meta.url), {
				type: 'module'
			});
			worker.postMessage(wasmData);
			if (!await new Promise(resolve => {
				worker.onmessage = (e) => resolve(e.data)
			})) {
				console.error('Starting wasmWorker failed');
			}
			return worker;
		})
	);
}
