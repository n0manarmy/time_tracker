const { contextBridge, ipcRenderer } = require('electron');
const path = require('path');

// spawn timer worker to send message very 1000ms to update the timer.
const timer_worker = new Worker(path.resolve(__dirname, 'current_time.js'));
// const rust_worker = new Worker(path.resolve(__dirname, 'rust_worker.js'));

contextBridge.exposeInMainWorld(
    'API', {
        send: (channel, data) => {
            let valid_channels = ["to-main"];
            if (valid_channels.includes(channel)) {
                ipcRenderer.send(channel, data);
            }
        },
        receive: (channel, func) => {
            let valid_channels = ["from-main"];
            if (valid_channels.includes(channe)) {
                ipcRenderer.on(channel, (event, ...args) => func(...args));
            }
        }
})