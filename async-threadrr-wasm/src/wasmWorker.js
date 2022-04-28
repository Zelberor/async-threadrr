console.log('wasmWorker created');

export function _dummyFunction() { }

onmessage = async (e) => {
	console.log('wasmWorker started');
	const pkg = await import('../../..');
	await pkg.default(e.data.module, e.data.memory);
	postMessage(true);
	pkg._runNoneBlocking(); p
}
