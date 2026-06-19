// modules/window/windowManager.js
const { BrowserWindow } = require('electron');
const path = require('path');

class WindowManager {
    constructor() {
        this.mainWindow = null;
    }

    createWindow() {
        this.mainWindow = new BrowserWindow({
            width: 1000,
            height: 700,
            autoHideMenuBar: true,
            webPreferences: {
                preload: path.join(__dirname, '../../preload.js'),
                contextIsolation: true,
                nodeIntegration: false,
            }
        });

        // Load the HTML file
        this.mainWindow.loadFile(path.join(__dirname, '../../resource', 'uibaru', 'monitor.html'));

        // Set fullscreen
        this.mainWindow.setFullScreen(true);

        // Handle window closed
        this.mainWindow.on('closed', () => {
            this.mainWindow = null;
        });

        console.log('Main window created');
    }

    getMainWindow() {
        return this.mainWindow;
    }

    closeWindow() {
        if (this.mainWindow) {
            this.mainWindow.close();
        }
    }

    isWindowCreated() {
        return this.mainWindow !== null;
    }
}

module.exports = WindowManager;