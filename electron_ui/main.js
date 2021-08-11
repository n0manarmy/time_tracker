console.log('init main.js')

// Modules to control application life and create native browser window
const { globalShortcut, app, BrowserWindow, protocol } = require('electron')
const path = require('path')

// Standard scheme must be registered before the app is ready
// https://gist.github.com/dbkr/e898624be6d53590ebf494521d868fec
protocol.registerSchemesAsPrivileged([{
    scheme: 'app',
    privileges: { standard: true, secure: true, supportFetchAPI: true },
}]);

function createWindow() {
	// Create the browser window.
	const mainWindow = new BrowserWindow({
		width: 800,
		height: 800,
		// maxHeight: 400,
		webPreferences: {
			nodeIntegrationInWorker: true,
			contextIsolation: false,
			nodeIntegration: true,
			preload: path.join(__dirname, 'preload.js')
		}
	})

	// mainWindow.webContents.openDevTools()

	// globalShortcut.register('f5', function () {
	// 	console.log('f5 is pressed')
	// 	mainWindow.reload()
	// })
	// globalShortcut.register('CommandOrControl+R', function () {
	// 	console.log('CommandOrControl+R is pressed')
	// 	mainWindow.reload()
	// })

	// and load the index.html of the app.
	mainWindow.loadFile('index.html')

	// Open the DevTools.
	// mainWindow.webContents.openDevTools()
}

// ipcMain.on("update_time", event => {
// 	// console.log("ipcMain.on caught update_time");
// 	event.sender.send("update_time_received");
// });

// ipcMain.on("update_time_in", event => {
// 	// console.log("ipcMain.on caught update_time_in");
// 	event.sender.send("time_in_received");
// });

// ipcMain.on("update_time_out", event => {
// 	// console.log("ipcMain.on caught update_time_out");
// 	event.sender.send("time_out_received");
// });


// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.whenReady().then(() => {
	createWindow()

	app.on('activate', function () {
		// On macOS it's common to re-create a window in the app when the
		// dock icon is clicked and there are no other windows open.
		if (BrowserWindow.getAllWindows().length === 0) createWindow()
	})
})

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', function () {
	if (process.platform !== 'darwin') app.quit()
})

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.