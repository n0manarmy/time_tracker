// This file is required by the index.html file and will
// be executed in the renderer process for that window.
// No Node.js APIs are available in this process because
// `nodeIntegration` is turned off. Use `preload.js` to
// selectively enable features needed in the rendering
// process.

console.log('init renderer.js')

const { ipcRenderer } = require('electron');
const path = require('path');

// spawn timer worker to send message very 1000ms to update the timer.
const timer_worker = new Worker(path.resolve(__dirname, 'current_time.js'));
const rust_worker = new Worker(path.resolve(__dirname, 'rust_worker.js'));
const time_in_button = document.getElementById('time_in_button');
const time_out_button = document.getElementById('time_out_button');

rust_worker.onmessage = function(event) {
    switch (event.data[0]) {
        case 'ack_get_current_time':
            document.getElementById('current_time_value').innerHTML = event.data[1];
            break;
        case 'ack_get_out':
            // console.log('ack_get_out', event.data[1])
            // console.log('ack_get_out', event.data[2])
            // console.log('ack_get_out', event.data[3])
            insert_into_time_table(event.data[1], event.data[2], event.data[3])
            break;
        case 'ack_get_in':
            // console.log('ack_get_in', event.data[1])
            // console.log('ack_get_in', event.data[2])
            // console.log('ack_get_in', event.data[3])
            insert_into_time_table(event.data[1], event.data[2], event.data[3])
            break;
        default:
            break;
    }
};

function insert_into_time_table(date, time_stamp, time_state) {
    let table = document.getElementById('time_table');
    let row = table.insertRow();

    let c0 = row.insertCell(0);
    let c1 = row.insertCell(1);
    let c2 = row.insertCell(2);
    
    let c0_text = document.createTextNode(date);
    let c1_text = document.createTextNode(time_stamp);
    let c2_text = document.createTextNode(time_state);
    
    c0.appendChild(c0_text);
    c1.appendChild(c1_text);
    c2.appendChild(c2_text);
}

// sends message from worker every 1000ms
timer_worker.onmessage = function() {
    // console.log('message recieved from worker');
    rust_worker.postMessage(['get_current_time']);
}

time_in_button.addEventListener('click', () => {
    rust_worker.postMessage(['get_in']);
    // ipcRenderer.send('update_time_in');
});

time_out_button.addEventListener('click', () => {
    rust_worker.postMessage(['get_out']);
    // ipcRenderer.send('update_time_out');
});

// // Catchers for buttons from main
// ipcRenderer.on('update_time_received', function(event, response) {
//     // console.log(event);
//     // let tz_offset = new Date().getTimezoneOffset();
//     // document.getElementById('current_time_value').innerHTML = Date().toLocaleString('en-US', { timezone: tz_offset });
//     rust_worker.postMessage(['get_wasm_time']);
// });

// ipcRenderer.on('time_in_received', function(event, response) {
//     console.log(event);

//     // let tz_offset = new Date().getTimezoneOffset();
//     // document.getElementById('current_time_value').innerHTML = Date().toLocaleString('en-US', { timezone: tz_offset });
// });

// ipcRenderer.on('time_out_received', function(event, response) {
//     console.log(event);
//     // let tz_offset = new Date().getTimezoneOffset();
//     // document.getElementById('current_time_value').innerHTML = Date().toLocaleString('en-US', { timezone: tz_offset });
// });