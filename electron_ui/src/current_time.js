// console.log('current_time.js loaded');

const sleep = async time => new Promise(r => setTimeout(r, time))

async function run_timer() {
    while (true) {
		// console.log('sleep called');
        self.postMessage('worker_update_time');
		await sleep(1000);
	}
}

run_timer();

// ~(async function main() {
// 	while (true) {
// 		console.log('sleep called');
// 		await sleep(1000);
// 	}
// })()