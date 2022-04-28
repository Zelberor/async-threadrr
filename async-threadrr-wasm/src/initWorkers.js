// Make sure workers stay in memory
let _workers;

export async function initWorkers(module, memory, blocking, amount) {
	if (amount <= 0) {
		throw new Error('Amount of workers must be > 0')
	}

	const wasmData = {
		module,
		memory,
		blocking
	}

	_workers = await Promise.all(
		Array.from({ length: amount }, async () => {
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
