console.log("init rust_worker");
const path = require('path');
const wasm_path = path.resolve(__dirname, '../rust_time_lib/pkg/time_tracker');

var wasm_interface = require(wasm_path);
var state;

onmessage = function(event) {
    // console.log(event);
    switch (event.data[0]) {
        case 'get_current_time':
            this.postMessage(['ack_get_current_time', wasm_interface.get_current_time()]);
            break;
        case 'get_in':
            console.log('get_in')
            state = wasm_interface.clock_in_out("IN");
            // console.log(state.date);
            // console.log(state.time_stamp);
            // console.log(state.time_state);
            this.postMessage(['ack_get_in', state.date, state.time_stamp, state.time_state]);
            break;
        case 'get_out':
            console.log('get_out')
            state = wasm_interface.clock_in_out("OUT");
            // console.log(state.date);
            // console.log(state.time_stamp);
            // console.log(state.time_state);
            this.postMessage(['ack_get_in', state.date, state.time_stamp, state.time_state]);
            break;
        default:
            break;
    }

}