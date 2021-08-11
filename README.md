# About
Time Tracker was originally a project I started during off time in COVID isolation. I was doing a lot of work from home and wanted a quick way to track time ins and time outs. Originally this was a command line app and then I built GTK+ in for a GUI. There are branches that have working GTK+ releases. 

I've since migrated this to using ElectronJS and Rust WASM. I've always had an interest in WASM, especially with Rust.

# Installation
The current installation procedure is to follow the Electron JS setup tutorial. Head into the electron_ui directory and run "npm start". There is a link to the Rust WASM library in the rust_worker.js file. This may not work within Windows since it uses Linux pathing. If the Rust WASM module is built than the Electron UI should track current time as well as time in and time out.

If This is not working, ensure you have installed and built rust_time_lib. Follow the Rust and the Rust WASM installation directions. Head into the rust_time_lib directory and run "wasm-pack build --target nodejs". This should build the package that gets called by the Electron UI.

# Future Developement
* Add - saving of current times
* Add - auto loading or manually loading saved times
* Modify - Time details based on increments defined in the settings (15min, 5min, 1min time worked.)