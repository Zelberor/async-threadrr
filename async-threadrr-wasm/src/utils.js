
export function numThreads() {
	return navigator.hardwareConcurrency ?? 4;
}
