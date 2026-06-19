// modules/ipc/ipcManager.js
const { ipcMain } = require('electron');
const { SerialPort } = require('serialport');

class IPCManager {
    constructor(database, serialManagerGetter, websocketManagerGetter, csvLoggerGetter) {
        this.database = database;
        // Accept either a lazy getter function or a direct reference
        this._getSerial    = typeof serialManagerGetter    === 'function' ? serialManagerGetter    : () => serialManagerGetter;
        this._getWebsocket = typeof websocketManagerGetter === 'function' ? websocketManagerGetter : () => websocketManagerGetter;
        this._getCsvLogger = typeof csvLoggerGetter        === 'function' ? csvLoggerGetter        : () => csvLoggerGetter;
    }

    // Convenience accessors
    get serialManager()    { return this._getSerial(); }
    get websocketManager() { return this._getWebsocket(); }
    get csvLogger()        { return this._getCsvLogger(); }

    setupHandlers() {
        this.setupDatabaseHandlers();
        this.setupSerialHandlers();
        this.setupCsvHandlers();
        console.log('IPC handlers setup complete');
    }

    setupDatabaseHandlers() {
        // User handlers
        ipcMain.handle('get-users', async () => {
            try {
                const users = await this.database.getAllUsers();
                return { success: true, data: users };
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        ipcMain.handle('insert-user', async (event, name, email) => {
            try {
                const result = await this.database.insertUser(name, email);
                return { success: true, id: result.insertId };
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        // Generic data handlers
        ipcMain.handle('post-data', async (event, table, data) => {
            try {
                const result = await this.database.postData(table, data);
                return { success: true, id: result.insertId };
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        ipcMain.handle('insert-data', async (event, table, data) => {
            try {
                const result = await this.database.postData(table, data);
                return { success: true, id: result.insertId };
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        ipcMain.handle('update-data', async (event, table, data, whereClause, whereParams) => {
            try {
                const result = await this.database.updateData(table, data, whereClause, whereParams);
                return { success: true, affectedRows: result.affectedRows };
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        ipcMain.handle('delete-data', async (event, table, whereClause, whereParams) => {
            try {
                const result = await this.database.deleteData(table, whereClause, whereParams);
                return { success: true, affectedRows: result.affectedRows };
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        ipcMain.handle('get-data-by-filters', async (event, table, filters, options) => {
            try {
                const result = await this.database.getDataByFilters(table, filters, options);
                return { success: true, data: result };
            } catch (err) {
                console.error("Error in get-data-by-filters handler:", err);
                return { success: false, error: err.message };
            }
        });
    }

    setupSerialHandlers() {
        // Get serial connection status
        ipcMain.handle('serial-get-status', async () => {
            try {
                if (this.serialManager) {
                    return { success: true, data: this.serialManager.getStatus() };
                } else {
                    return { success: false, error: 'Serial manager not initialized' };
                }
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        // Force reconnection
        ipcMain.handle('serial-force-reconnect', async () => {
            try {
                if (this.serialManager) {
                    await this.serialManager.forceReconnect();
                    return { success: true, message: 'Reconnection initiated' };
                } else {
                    return { success: false, error: 'Serial manager not initialized' };
                }
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        // Disconnect serial connection
        ipcMain.handle('serial-disconnect', async () => {
            try {
                if (this.serialManager) {
                    await this.serialManager.disconnect();
                    return { success: true, message: 'Disconnected successfully' };
                } else {
                    return { success: false, error: 'Serial manager not initialized' };
                }
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        // Scan for better ports (internal auto-switch)
        ipcMain.handle('serial-scan-ports', async () => {
            try {
                if (this.serialManager) {
                    await this.serialManager.scanForBetterPorts();
                    return { success: true, message: 'Port scanning initiated' };
                } else {
                    return { success: false, error: 'Serial manager not initialized' };
                }
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        // List all available serial ports (for frontend dropdown)
        ipcMain.handle('serial-list-ports', async () => {
            try {
                const ports = await SerialPort.list();
                const portList = ports.map(p => ({
                    path:         p.path,
                    manufacturer: p.manufacturer || null,
                    vendorId:     p.vendorId     || null,
                    productId:    p.productId    || null,
                    serialNumber: p.serialNumber || null,
                }));
                return { success: true, ports: portList };
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        // Toggle dynamic port switching
        ipcMain.handle('serial-toggle-dynamic-switching', async (event, enabled) => {
            try {
                if (this.serialManager) {
                    this.serialManager.setDynamicPortSwitching(enabled);
                    return { success: true, message: `Dynamic switching ${enabled ? 'enabled' : 'disabled'}` };
                } else {
                    return { success: false, error: 'Serial manager not initialized' };
                }
            } catch (err) {
                return { success: false, error: err.message };
            }
        });

        // Send data to serial device
        ipcMain.handle('serial-send-data', async (event, data) => {
            try {
                if (this.serialManager) {
                    this.serialManager.sendData(data);
                    return { success: true, message: 'Data sent successfully' };
                } else {
                    return { success: false, error: 'Serial manager not initialized' };
                }
            } catch (err) {
                return { success: false, error: err.message };
            }
        });
    }  // end setupSerialHandlers

    setupCsvHandlers() {
        // ── Start recording ──────────────────────────────────────────────────────
        ipcMain.handle('csv-start', async () => {
            const logger = this.csvLogger;
            if (!logger) return { success: false, error: 'CSV logger not initialized' };
            // Include current port name in the filename
            const serial = this.serialManager;
            const status = serial ? serial.getStatus() : null;
            const port   = (status && status.currentPortPath) || 'NDIR';
            return logger.start(port);
        });

        // ── Pause recording ──────────────────────────────────────────────────────
        ipcMain.handle('csv-pause', async () => {
            const logger = this.csvLogger;
            if (!logger) return { success: false, error: 'CSV logger not initialized' };
            return logger.pause();
        });

        // ── Resume recording ─────────────────────────────────────────────────────
        ipcMain.handle('csv-resume', async () => {
            const logger = this.csvLogger;
            if (!logger) return { success: false, error: 'CSV logger not initialized' };
            return logger.resume();
        });

        // ── Stop recording ───────────────────────────────────────────────────────
        ipcMain.handle('csv-stop', async () => {
            const logger = this.csvLogger;
            if (!logger) return { success: false, error: 'CSV logger not initialized' };
            return logger.stop();
        });

        // ── Get status ───────────────────────────────────────────────────────────
        ipcMain.handle('csv-get-status', async () => {
            const logger = this.csvLogger;
            if (!logger) return { success: false, error: 'CSV logger not initialized' };
            return { success: true, ...logger.getStatus() };
        });
    }
}

module.exports = IPCManager;