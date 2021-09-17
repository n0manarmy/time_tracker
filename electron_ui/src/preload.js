// All of the Node.js APIs are available in the preload process.
// It has the same sandbox as a Chrome extension.
// console.log("init preload.js");

// window.addEventListener('DOMContentLoaded', () => {

// 	const replaceText = (selector, text) => {
// 		const element = document.getElementById(selector)
// 		if (element) element.innerText = text
// 	}

// 	for (const type of ['chrome', 'node', 'electron']) {
// 		replaceText(`${type}-version`, process.versions[type])
// 	}
// })

// console.log('init preload.js');

// const { ipcRenderer } = require('electron');
// const path = require('path');

// // spawn timer worker to send message very 1000ms to update the timer.
// const timer_worker = new Worker(path.resolve(__dirname, 'current_time.js'));
// const rust_worker = new Worker(path.resolve(__dirname, 'rust_worker.js'));

// rust_worker.postMessage('load_log_file');