// SerialCommunicator.js
const { SerialPort, ReadlineParser } = require('serialport');

// Console colors for better debugging visibility
const colors = {
    reset: '\x1b[0m',
    bright: '\x1b[1m',
    dim: '\x1b[2m',
    red: '\x1b[31m',
    green: '\x1b[32m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    magenta: '\x1b[35m',
    cyan: '\x1b[36m',
    white: '\x1b[37m',
    bgRed: '\x1b[41m',
    bgGreen: '\x1b[42m',
    bgYellow: '\x1b[43m',
    bgBlue: '\x1b[44m',
    bgMagenta: '\x1b[45m',
    bgCyan: '\x1b[46m'
};

// Enhanced logging utility
const logger = {
    info: (message, data = null) => {
        console.log(`${colors.cyan}[INFO]${colors.reset} ${colors.white}${message}${colors.reset}`);
        if (data) console.log(`${colors.dim}${JSON.stringify(data, null, 2)}${colors.reset}`);
    },
    success: (message, data = null) => {
        console.log(`${colors.green}[SUCCESS]${colors.reset} ${colors.bright}${message}${colors.reset}`);
        if (data) console.log(`${colors.dim}${JSON.stringify(data, null, 2)}${colors.reset}`);
    },
    warning: (message, data = null) => {
        console.log(`${colors.yellow}[WARN]${colors.reset} ${colors.yellow}${message}${colors.reset}`);
        if (data) console.log(`${colors.dim}${JSON.stringify(data, null, 2)}${colors.reset}`);
    },
    error: (message, error = null) => {
        console.log(`${colors.red}[ERROR]${colors.reset} ${colors.red}${message}${colors.reset}`);
        if (error) {
            if (error.stack) {
                console.log(`${colors.dim}${error.stack}${colors.reset}`);
            } else {
                console.log(`${colors.dim}${JSON.stringify(error, null, 2)}${colors.reset}`);
            }
        }
    },
    debug: (message, data = null) => {
        console.log(`${colors.magenta}[DEBUG]${colors.reset} ${colors.magenta}${message}${colors.reset}`);
        if (data) console.log(`${colors.dim}${JSON.stringify(data, null, 2)}${colors.reset}`);
    },
    section: (title, content = null) => {
        console.log(`${colors.bgBlue}${colors.white} ${title} ${colors.reset}`);
        if (content) console.log(`${colors.dim}${typeof content === 'string' ? content : JSON.stringify(content, null, 2)}${colors.reset}`);
        console.log(`${colors.blue}${'='.repeat(50)}${colors.reset}`);
    }
};

class SerialCommunicator {
    constructor(config = {}, dbInstance, windowInstance, csvLoggerInstance = null) {
        logger.section('SerialCommunicator Constructor');
        logger.debug('Received config', config);
        logger.debug('DB instance type', dbInstance ? dbInstance.constructor.name : 'null');
        logger.debug('Window instance', windowInstance ? 'provided' : 'null');

        this.config = {
            portPath: null,
            baudRate: 9600,
            dataType: 'json-object',
            lineDelimiter: '\r\n',
            csvDelimiter: ',',
            fieldMapping: [],
            dbTableName: null,
            requiredFields: [],
            fieldsToEncrypt: [],
            autoReconnect: true,
            reconnectDelay: 3000,
            maxReconnectAttempts: 10,
            connectionTimeout: 5000,
            portScanInterval: 15000,
            enableDynamicPortSwitching: true,
            ...config
        };

        logger.success('Configuration merged successfully', this.config);

        this.db         = dbInstance;
        this.mainWindow = windowInstance;
        this.csvLogger  = csvLoggerInstance || null;   // ← CSV recording
        this.arduinoPort = null;
        this.parser = null;
        this.isConnecting = false;
        this.isIntentionallyDisconnected = false;
        this.reconnectAttempts = 0;
        this.reconnectTimer = null;
        this.connectionCheckInterval = null;
        this.portScanTimer = null;
        this.lastDataReceived = Date.now();
        this.currentPortPath = null;
        this.isConnectedToPotentialPort = false;

        // Connection states
        this.connectionStates = {
            DISCONNECTED: 'disconnected',
            CONNECTING: 'connecting',
            CONNECTED: 'connected',
            RECONNECTING: 'reconnecting',
            SWITCHING_PORTS: 'switching_ports',
            ERROR: 'error'
        };
        this.currentState = this.connectionStates.DISCONNECTED;
    }

    setMainWindow(windowInstance) {
        this.mainWindow = windowInstance;
        logger.info('Main window instance updated');
    }

    // Get current connection status
    getStatus() {
        const status = {
            state: this.currentState,
            isConnected: this.isConnected(),
            port: this.getPortInfo(),
            reconnectAttempts: this.reconnectAttempts,
            maxReconnectAttempts: this.config.maxReconnectAttempts,
            autoReconnect: this.config.autoReconnect,
            lastDataReceived: this.lastDataReceived,
            isConnectedToPotentialPort: this.isConnectedToPotentialPort,
            currentPortPath: this.currentPortPath
        };
        
        logger.debug('Current status requested', status);
        return status;
    }

    // Set connection state and notify renderer
    _setState(newState, message = '') {
        if (this.currentState !== newState) {
            logger.info(`State Change: ${this.currentState} → ${newState}`, message ? { message } : null);
            this.currentState = newState;
            this._sendToRenderer('serial-port-status', {
                state:     newState,
                message:   message,
                port:      this.currentPortPath,   // ← Bug 1 fix: always include port path
                timestamp: new Date().toISOString()
            });
        }
    }

    async connect() {
        if (this.isConnecting) {
            logger.warning('Connection already in progress, skipping...');
            return;
        }

        logger.section('Starting Connection Process');
        this.isConnecting = true;
        this.isIntentionallyDisconnected = false;
        this._setState(this.connectionStates.CONNECTING, 'Initiating connection...');

        try {
            if (!this.config.portPath) {
                logger.info('No port specified, starting auto-detection...');
                await this._autoDetectAndConnect();
            } else {
                logger.info(`Connecting to specified port: ${this.config.portPath}`);
                await this._connectToPort(this.config.portPath);
            }

            // Start periodic port scanning if enabled
            if (this.config.enableDynamicPortSwitching) {
                logger.info('Starting port scanning for dynamic switching');
                this._startPortScanning();
            }
        } catch (error) {
            logger.error('Connection failed', error);
            this._setState(this.connectionStates.ERROR, `Connection failed: ${error.message}`);
            this._sendToRenderer('serial-port-error', `Connection failed: ${error.message}`);

            // Start reconnection if auto-reconnect is enabled
            if (this.config.autoReconnect && !this.isIntentionallyDisconnected) {
                logger.info('Scheduling reconnection attempt');
                this._scheduleReconnection();
            }
        } finally {
            this.isConnecting = false;
        }
    }

    // Start periodic port scanning for better connections
    _startPortScanning() {
        if (this.portScanTimer) {
            clearInterval(this.portScanTimer);
        }

        this.portScanTimer = setInterval(async () => {
            try {
                await this._scanForBetterPort();
            } catch (error) {
                logger.error('Error during port scanning', error);
            }
        }, this.config.portScanInterval);

        logger.success(`Port scanning started (interval: ${this.config.portScanInterval}ms)`);
    }

    // Stop port scanning
    _stopPortScanning() {
        if (this.portScanTimer) {
            clearInterval(this.portScanTimer);
            this.portScanTimer = null;
            logger.info('Port scanning stopped');
        }
    }

    // Scan for better ports and switch if found
    async _scanForBetterPort() {
        // Don't scan if we're already connected to a potential port or if we're in the middle of connecting
        if (this.isConnectedToPotentialPort || this.isConnecting || !this.isConnected()) {
            return;
        }

        try {
            logger.debug('Scanning for better Arduino/ESP32 ports...');
            const ports = await SerialPort.list();

            // Look for potential ports
            const potentialPorts = ports.filter(p => {
                const manufacturer = (p.manufacturer || '').toLowerCase();
                const vendorId = p.vendorId;

                return manufacturer.includes('arduino') ||
                    manufacturer.includes('esp32') ||
                    manufacturer.includes('silicon labs') ||
                    manufacturer.includes('ch340') ||
                    manufacturer.includes('ftdi') ||
                    manufacturer.includes('prolific') ||
                    vendorId === '10C4' || // Silicon Labs
                    vendorId === '1A86' || // CH340
                    vendorId === '0403' || // FTDI
                    vendorId === '2341';   // Arduino
            });

            // Check if we found a potential port that's different from current
            if (potentialPorts.length > 0) {
                const bestPotentialPort = potentialPorts[0].path;

                if (bestPotentialPort !== this.currentPortPath) {
                    logger.warning(`Better port detected: ${bestPotentialPort} (current: ${this.currentPortPath})`);

                    this._sendToRenderer('serial-port-status', {
                        state: 'better_port_detected',
                        message: `Better port detected: ${bestPotentialPort}`,
                        currentPort: this.currentPortPath,
                        newPort: bestPotentialPort,
                        timestamp: new Date().toISOString()
                    });

                    await this._switchToPort(bestPotentialPort);
                }
            }
        } catch (error) {
            logger.error('Error scanning for better ports', error);
        }
    }

    // Switch to a new port
    async _switchToPort(newPortPath) {
        try {
            logger.section(`Port Switching: ${this.currentPortPath} → ${newPortPath}`);
            this._setState(this.connectionStates.SWITCHING_PORTS, `Switching to better port: ${newPortPath}`);

            // Close current connection
            await this._closeConnection();

            // Connect to new port
            await this._connectToPort(newPortPath);

            this._sendToRenderer('serial-port-switched', {
                oldPort: this.currentPortPath,
                newPort: newPortPath,
                timestamp: new Date().toISOString()
            });

            logger.success(`Successfully switched to port: ${newPortPath}`);
        } catch (error) {
            logger.error(`Failed to switch to port ${newPortPath}`, error);
            this._sendToRenderer('serial-port-error', `Failed to switch to port ${newPortPath}: ${error.message}`);

            // Try to reconnect to original port or find any available port
            if (this.config.autoReconnect && !this.isIntentionallyDisconnected) {
                this._scheduleReconnection();
            }
        }
    }

    // Force reconnection (can be called from renderer)
    async forceReconnect() {
        logger.section('Manual Reconnection Initiated');
        this._sendToRenderer('serial-reconnect-status', {
            status: 'manual_reconnect_started',
            message: 'Manual reconnection initiated...'
        });

        // Cancel any existing reconnection timer
        this._cancelReconnection();

        // Reset attempts for manual reconnection
        this.reconnectAttempts = 0;
        this.isIntentionallyDisconnected = false;

        // Close existing connection if any
        await this._closeConnection();

        // Start fresh connection attempt
        await this.connect();
    }

    // Intentional disconnect (stops auto-reconnection)
    async disconnect() {
        logger.section('Intentional Disconnect Requested');
        this.isIntentionallyDisconnected = true;
        this._cancelReconnection();
        this._stopConnectionMonitoring();
        this._stopPortScanning();
        await this._closeConnection();
        this._setState(this.connectionStates.DISCONNECTED, 'Intentionally disconnected');
    }

    async _autoDetectAndConnect() {
        try {
            logger.info('Scanning for Arduino/ESP32 devices...');
            const ports = await SerialPort.list();

            const portSummary = ports.map(p => ({
                path: p.path,
                manufacturer: p.manufacturer,
                vendorId: p.vendorId,
                productId: p.productId
            }));
            logger.debug('Available ports', portSummary);

            // Look for common Arduino/ESP32 identifiers
            const potentialPorts = ports.filter(p => {
                const manufacturer = (p.manufacturer || '').toLowerCase();
                const vendorId = p.vendorId;

                return manufacturer.includes('arduino') ||
                    manufacturer.includes('esp32') ||
                    manufacturer.includes('silicon labs') ||
                    manufacturer.includes('ch340') ||
                    manufacturer.includes('ftdi') ||
                    manufacturer.includes('prolific') ||
                    vendorId === '10C4' || // Silicon Labs
                    vendorId === '1A86' || // CH340
                    vendorId === '0403' || // FTDI
                    vendorId === '2341';   // Arduino
            });

            if (potentialPorts.length > 0) {
                logger.success('Found potential Arduino/ESP32 ports', potentialPorts.map(p => p.path));
                this.isConnectedToPotentialPort = true;
                await this._connectToPort(potentialPorts[0].path);
            } else if (ports.length > 0) {
                logger.warning('No obvious Arduino/ESP32 ports found, trying first available port...');
                this.isConnectedToPotentialPort = false;
                await this._connectToPort(ports[0].path);
            } else {
                throw new Error('No serial ports available');
            }
        } catch (error) {
            logger.error('Error during auto-detection', error);
            throw error;
        }
    }

    async _connectToPort(portPath) {
        return new Promise((resolve, reject) => {
            logger.info(`Connecting to: ${portPath} @ ${this.config.baudRate} baud`);
            this._setState(this.connectionStates.CONNECTING, `Connecting to ${portPath}...`);

            // Close existing connection if any
            if (this.arduinoPort && this.arduinoPort.isOpen) {
                this.arduinoPort.close();
            }

            // Create connection timeout
            const connectionTimeout = setTimeout(() => {
                if (this.arduinoPort) {
                    this.arduinoPort.close();
                }
                reject(new Error(`Connection timeout after ${this.config.connectionTimeout}ms`));
            }, this.config.connectionTimeout);

            this.arduinoPort = new SerialPort({
                path: portPath,
                baudRate: this.config.baudRate,
                autoOpen: false
            });

            // Set up event listeners before opening
            this.arduinoPort.on('open', () => {
                clearTimeout(connectionTimeout);
                logger.success(`Port ${portPath} opened successfully`);
                this.currentPortPath = portPath;
                this._setState(this.connectionStates.CONNECTED, `Connected to ${portPath}`);
                this.reconnectAttempts = 0;
                this.lastDataReceived = Date.now();

                // Set up parser after successful connection
                this.parser = this.arduinoPort.pipe(new ReadlineParser({
                    delimiter: this.config.lineDelimiter
                }));
                this.parser.on('data', data => this._handleData(data));

                // Start connection monitoring
                this._startConnectionMonitoring();

                resolve();
            });

            this.arduinoPort.on('error', (err) => {
                clearTimeout(connectionTimeout);
                logger.error(`Serial Error on ${portPath}`, err);
                this._setState(this.connectionStates.ERROR, `Port Error: ${err.message}`);
                this._sendToRenderer('serial-port-error', `Port Error: ${err.message}`);
                this.arduinoPort = null;
                this.currentPortPath = null;
                reject(err);
            });

            this.arduinoPort.on('close', () => {
                clearTimeout(connectionTimeout);
                logger.warning(`Port ${portPath} closed`);

                if (!this.isIntentionallyDisconnected) {
                    this._setState(this.connectionStates.DISCONNECTED, `Connection lost: ${portPath}`);
                    this._sendToRenderer('serial-connection-lost', {
                        port: portPath,
                        timestamp: new Date().toISOString(),
                        reconnectAttempts: this.reconnectAttempts
                    });
                }

                this.arduinoPort = null;
                this.parser = null;
                this.currentPortPath = null;
                this.isConnectedToPotentialPort = false;
                this._stopConnectionMonitoring();
                this._stopPortScanning();

                // Attempt reconnection if not intentionally closed and auto-reconnect is enabled
                if (this.config.autoReconnect && !this.isIntentionallyDisconnected) {
                    this._scheduleReconnection();
                }
            });

            // Now open the port
            this.arduinoPort.open((err) => {
                if (err) {
                    clearTimeout(connectionTimeout);
                    logger.error(`Failed to open port ${portPath}`, err);
                    reject(err);
                }
            });
        });
    }

    _scheduleReconnection() {
        if (this.reconnectAttempts >= this.config.maxReconnectAttempts) {
            logger.error(`Max reconnection attempts (${this.config.maxReconnectAttempts}) reached. Stopping auto-reconnection.`);
            this._setState(this.connectionStates.ERROR, `Max reconnection attempts reached (${this.config.maxReconnectAttempts})`);
            this._sendToRenderer('serial-reconnect-status', {
                status: 'max_attempts_reached',
                attempts: this.reconnectAttempts,
                maxAttempts: this.config.maxReconnectAttempts
            });
            return;
        }

        const delaySeconds = this.config.reconnectDelay / 1000;
        const attemptInfo = `Attempt ${this.reconnectAttempts + 1}/${this.config.maxReconnectAttempts}`;
        logger.warning(`Reconnecting in ${delaySeconds}s... (${attemptInfo})`);
        
        this._setState(this.connectionStates.RECONNECTING, `Reconnecting in ${delaySeconds}s... (${attemptInfo})`);

        this._sendToRenderer('serial-reconnect-status', {
            status: 'scheduled',
            attempts: this.reconnectAttempts,
            maxAttempts: this.config.maxReconnectAttempts,
            delay: this.config.reconnectDelay
        });

        this.reconnectTimer = setTimeout(() => {
            this.reconnectAttempts++;
            logger.info(`Reconnection attempt ${this.reconnectAttempts}/${this.config.maxReconnectAttempts}`);

            this._sendToRenderer('serial-reconnect-status', {
                status: 'attempting',
                attempts: this.reconnectAttempts,
                maxAttempts: this.config.maxReconnectAttempts
            });

            this.connect();
        }, this.config.reconnectDelay);
    }

    _cancelReconnection() {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
            logger.info('Reconnection timer cancelled');
        }
    }

    _startConnectionMonitoring() {
        // Monitor connection health by checking for regular data
        this.connectionCheckInterval = setInterval(() => {
            const timeSinceLastData = Date.now() - this.lastDataReceived;

            // If no data received for more than 30 seconds, consider connection potentially dead
            if (timeSinceLastData > 30000 && this.isConnected()) {
                const seconds = Math.floor(timeSinceLastData / 1000);
                logger.warning(`No data received for ${seconds}s. Connection may be unstable.`);
                this._sendToRenderer('serial-port-error', `No data received for ${seconds} seconds`);
            }
        }, 10000); // Check every 10 seconds

        logger.debug('Connection monitoring started');
    }

    _stopConnectionMonitoring() {
        if (this.connectionCheckInterval) {
            clearInterval(this.connectionCheckInterval);
            this.connectionCheckInterval = null;
            logger.debug('Connection monitoring stopped');
        }
    }

    async _closeConnection() {
        return new Promise((resolve) => {
            if (this.arduinoPort && this.arduinoPort.isOpen) {
                this.arduinoPort.close(err => {
                    if (err) {
                        logger.error('Error closing serial port', err);
                    } else {
                        logger.success('Serial port closed successfully');
                    }
                    this.arduinoPort = null;
                    this.parser = null;
                    this.currentPortPath = null;
                    resolve();
                });
            } else {
                this.arduinoPort = null;
                this.parser = null;
                this.currentPortPath = null;
                resolve();
            }
        });
    }

    _handleData(rawString) {
        // Strip stray \r — ufmt sends \n only; some USB-serial chips add \r
        const trimmedData = rawString.replace(/\r/g, '').trim();
        this.lastDataReceived = Date.now();

        // Clean, organized data reception logging
        logger.section('Serial Data Received');
        logger.debug('Raw data length', rawString.length);
        logger.debug('Raw data (escaped)', JSON.stringify(rawString));
        logger.debug('Trimmed data', JSON.stringify(trimmedData));
        logger.debug('Data type config', this.config.dataType);
        logger.debug('Line delimiter', JSON.stringify(this.config.lineDelimiter));
        logger.debug('Current port', this.currentPortPath);

        this._sendToRenderer('serial-data-received', {
            raw: trimmedData,
            timestamp: new Date().toLocaleTimeString(),
            port: this.currentPortPath
        });

        // Write to CSV if recording
        if (this.csvLogger) {
            this.csvLogger.writeLine(trimmedData);
        }

        // Skip empty data
        if (!trimmedData) {
            logger.warning('Skipping empty data');
            return;
        }

        let parsedData, dataForDb = {};

        try {
            switch (this.config.dataType) {
                case 'json-object':
                    logger.info('Parsing as JSON object...');
                    dataForDb = JSON.parse(trimmedData);
                    logger.success('JSON object parsed successfully', dataForDb);
                    break;

                case 'json-array':
                    logger.info('Parsing as JSON array...');
                    parsedData = JSON.parse(trimmedData);
                    if (!Array.isArray(parsedData) || parsedData.length !== this.config.fieldMapping.length) {
                        throw new Error(`Array data mismatch. Expected ${this.config.fieldMapping.length} items, got ${parsedData.length}`);
                    }
                    this.config.fieldMapping.forEach((field, i) => {
                        dataForDb[field] = parsedData[i];
                    });
                    logger.success('JSON array parsed successfully', dataForDb);
                    break;

                case 'csv':
                    logger.info('Parsing as CSV...');
                    parsedData = trimmedData.split(this.config.csvDelimiter);
                    if (parsedData.length !== this.config.fieldMapping.length) {
                        throw new Error(`CSV data mismatch. Expected ${this.config.fieldMapping.length} items, got ${parsedData.length}`);
                    }
                    this.config.fieldMapping.forEach((field, i) => {
                        const val = parsedData[i].trim();
                        dataForDb[field] = !isNaN(parseFloat(val)) && isFinite(val) && val !== '' ? Number(val) : val;
                    });
                    logger.success('CSV parsed successfully', dataForDb);
                    break;

                case 'raw':
                    logger.info('Storing as raw data...');
                    dataForDb = { raw_data: trimmedData, timestamp: new Date().toISOString() };
                    logger.success('Raw data stored', dataForDb);
                    break;

                default:
                    throw new Error(`Unsupported dataType: ${this.config.dataType}`);
            }

            // Clean validation logging
            logger.section('Data Processing');
            logger.debug('Processed data (before validation)', dataForDb);
            logger.debug('Required fields', this.config.requiredFields);
            logger.debug('DB table name', this.config.dbTableName);
            logger.debug('DB instance exists', !!this.db);

            // Validate required fields
            if (this.config.requiredFields.length > 0) {
                logger.info('Validating required fields...');
                for (const field of this.config.requiredFields) {
                    const fieldValue = dataForDb[field];
                    const isEmpty = fieldValue === undefined || fieldValue === null || String(fieldValue).trim() === '';
                    
                    if (isEmpty) {
                        logger.error(`Data missing required field '${field}', skipping database insert`);
                        return;
                    } else {
                        logger.debug(`Field '${field}' validated`, { value: fieldValue });
                    }
                }
                logger.success('All required fields validated');
            } else {
                logger.info('No required fields to validate');
            }

            // Save to Database
            if (this.config.dbTableName && this.db) {
                logger.info('Proceeding to save to database...');
                this._saveToDatabase(dataForDb);
            } else {
                logger.warning('Database save skipped', {
                    hasTableName: !!this.config.dbTableName,
                    hasDB: !!this.db,
                    tableName: this.config.dbTableName
                });
            }

        } catch (err) {
            logger.section('Data Handling Error', 'red');
            logger.error('Parsing failed', {
                message: err.message,
                rawData: trimmedData,
                dataType: this.config.dataType,
                config: this.config
            });

            this._sendToRenderer('serial-port-error', `Data Error: ${err.message}`);
        }
    }

    _saveToDatabase(dataForDb) {
        let dataToInsert = { ...dataForDb };

        // Handle encryption if configured
        if (this.db.encrypt && this.config.fieldsToEncrypt && this.config.fieldsToEncrypt.length > 0) {
            logger.info('Encrypting fields', this.config.fieldsToEncrypt);
            for (const field of this.config.fieldsToEncrypt) {
                if (dataToInsert.hasOwnProperty(field) && dataToInsert[field] !== null && dataToInsert[field] !== undefined) {
                    try {
                        dataToInsert[field] = this.db.encrypt(String(dataToInsert[field]));
                        logger.success(`Field '${field}' encrypted`);
                    } catch (encError) {
                        logger.error(`Error encrypting field '${field}'`, encError);
                        this._sendToRenderer('serial-port-error', `Encryption Error for ${field}: ${encError.message}`);
                    }
                }
            }
        }

        logger.debug('Final data for DB insert', dataToInsert);

        this.db.postData(this.config.dbTableName, dataToInsert)
            .then(res => {
                logger.success(`DB Insert successful (${this.config.dbTableName}): ID ${res.insertId}`);
                this._sendToRenderer('database-insert-success', {
                    table: this.config.dbTableName,
                    insertId: res.insertId,
                    data: dataForDb,
                    port: this.currentPortPath
                });
            })
            .catch(err => {
                logger.error(`DB Insert Error (${this.config.dbTableName})`, err);
                this._sendToRenderer('serial-port-error', `DB Insert: ${err.message}`);
            });
    }

    // Method to send data to Arduino/ESP32
    sendData(data) {
        if (this.arduinoPort && this.arduinoPort.isOpen) {
            this.arduinoPort.write(data + '\n', (err) => {
                if (err) {
                    logger.error('Error sending data', err);
                    this._sendToRenderer('serial-port-error', `Send Error: ${err.message}`);
                } else {
                    logger.success('Data sent', { data, port: this.currentPortPath });
                    this._sendToRenderer('serial-data-sent', {
                        data: data,
                        port: this.currentPortPath,
                        timestamp: new Date().toISOString()
                    });
                }
            });
        } else {
            logger.warning('Cannot send data: port not open');
            this._sendToRenderer('serial-port-error', 'Cannot send data: port not connected');
        }
    }

    // Get connection status
    isConnected() {
        return this.arduinoPort && this.arduinoPort.isOpen;
    }

    // Get current port info
    getPortInfo() {
        if (this.arduinoPort) {
            return {
                path: this.arduinoPort.path,
                baudRate: this.arduinoPort.baudRate,
                isOpen: this.arduinoPort.isOpen,
                isPotentialPort: this.isConnectedToPotentialPort
            };
        }
        return null;
    }

    // Manual method to trigger port scanning
    async scanForBetterPorts() {
        if (this.config.enableDynamicPortSwitching) {
            await this._scanForBetterPort();
        } else {
            logger.warning('Dynamic port switching is disabled');
        }
    }

    // Enable/disable dynamic port switching
    setDynamicPortSwitching(enabled) {
        this.config.enableDynamicPortSwitching = enabled;

        if (enabled && this.isConnected()) {
            this._startPortScanning();
        } else {
            this._stopPortScanning();
        }

        logger.info(`Dynamic port switching ${enabled ? 'enabled' : 'disabled'}`);
    }

    _sendToRenderer(channel, data) {
        if (this.mainWindow && this.mainWindow.webContents) {
            this.mainWindow.webContents.send(channel, data);
            logger.debug(`Sent to renderer: ${channel}`, data);
        } else {
            logger.warning(`Cannot send to renderer (${channel}): no main window`, data);
        }
    }

    async close() {
        logger.section('SerialCommunicator Closing');
        this.isIntentionallyDisconnected = true;
        this._cancelReconnection();
        this._stopConnectionMonitoring();
        this._stopPortScanning();
        await this._closeConnection();
        this._setState(this.connectionStates.DISCONNECTED, 'SerialCommunicator closed');
        logger.success('SerialCommunicator closed successfully');
    }
}

module.exports = SerialCommunicator;