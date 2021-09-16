// This file is required by the index.html file and will
// be executed in the renderer process for that window.
// No Node.js APIs are available in this process because
// `nodeIntegration` is turned off. Use `preload.js` to
// selectively enable features needed in the rendering
// process.

// const { ipcRenderer } = require('electron');

// const path = require('path');

// // spawn timer worker to send message very 1000ms to update the timer.
// const timer_worker = new Worker(path.resolve(__dirname, 'current_time.js'));
// const rust_worker = new Worker(path.resolve(__dirname, 'rust_worker.js'));

const time_in_button = document.getElementById('time_in_button');
const time_out_button = document.getElementById('time_out_button');

window.api.receive('from-main', (data) => {
    console.log('from-main-received')
})

// ipcRenderer.on('update-current-time', (event, arg) => {
//     console.log(arg)
//     console.log(event)
// })

// rust_worker.onmessage = function(event) {
//     switch (event.data[0]) {
//         // case 'ack_load_log_file':
//         //     console.log('ack_load_log_file');
//         //     console.log(event.data[1]);
//         case 'ack_get_current_time':
//             document.getElementById('current_time_value').innerHTML = event.data[1];
//             break;
//         case 'ack_get_out':
//             // console.log('ack_get_out', event.data[1])
//             // console.log('ack_get_out', event.data[2])
//             // console.log('ack_get_out', event.data[3])
//             insert_into_time_table(event.data[1], event.data[2], event.data[3])
//             break;
//         case 'ack_get_in':
//             // console.log('ack_get_in', event.data[1])
//             // console.log('ack_get_in', event.data[2])
//             // console.log('ack_get_in', event.data[3])
//             insert_into_time_table(event.data[1], event.data[2], event.data[3])
//             break;
//         default:
//             break;
//     }
// };

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

// // sends message from worker every 1000ms
// timer_worker.onmessage = function() {
//     // console.log('message recieved from worker');
//     rust_worker.postMessage(['get_current_time']);
// }

// time_in_button.addEventListener('click', () => {
//     rust_worker.postMessage(['get_in']);
//     // ipcRenderer.send('update_time_in');
// });

// time_out_button.addEventListener('click', () => {
//     rust_worker.postMessage(['get_out']);
//     // ipcRenderer.send('update_time_out');
// });

// Catchers for buttons from main
// ipcRenderer.on('main_load_log_file', function(event, response) {
//     console.log('main_load_log_file');
//     // let tz_offset = new Date().getTimezoneOffset();
//     // document.getElementById('current_time_value').innerHTML = Date().toLocaleString('en-US', { timezone: tz_offset });
//     rust_worker.postMessage('load_log_file');
// });

// rust_worker.postMessage(['load_log_file']);
