// modules/serial/serialManager.js
const SerialCommunicator = require('../../App/lib/com/serialCommunicator');

class SerialManager {
    constructor(database, mainWindow, csvLogger = null) {
        console.log('=== SerialManager Constructor ===');
        console.log('Database provided:', !!database);
        console.log('MainWindow provided:', !!mainWindow);
        console.log('CsvLogger provided:', !!csvLogger);

        this.database   = database;
        this.mainWindow = mainWindow;
        this.csvLogger  = csvLogger;
        this.serialCommunicator = null;
        this.config = this.getSerialConfig();

        console.log('Final SerialManager config:', JSON.stringify(this.config, null, 2));
        console.log('=== End SerialManager Constructor ===');
    }

    getSerialConfig() {
        // The Mega firmware uses ufmt::uwriteln! which appends '\n' (LF only).
        // If .env has SERIAL_LINE_DELIMITER=\n  → we parse it to actual LF.
        // If .env has SERIAL_LINE_DELIMITER=\r\n → we parse it to actual CRLF.
        const parseLineDelimiter = (delimiter) => {
            if (!delimiter) return '\n'; // default: LF only (matches ufmt)

            // Replace escape sequences from .env literal strings
            return delimiter
                .replace(/\\r/g, '\r')
                .replace(/\\n/g, '\n')
                .replace(/\\t/g, '\t');
        };

        const rawDelimiter = process.env.SERIAL_LINE_DELIMITER || '\\n';
        const parsedDelimiter = parseLineDelimiter(rawDelimiter);

        const config = {
            portPath: process.env.SERIAL_PORT || null,
            baudRate: process.env.SERIAL_BAUDRATE ? parseInt(process.env.SERIAL_BAUDRATE, 10) : 9600,
            lineDelimiter: parsedDelimiter,
            dataType: process.env.SERIAL_DATA_TYPES || 'json-object',
            dbTableName: process.env.SERIAL_DB_TABLE_NAME || 'sensors_data',
            requiredFields: process.env.SERIAL_REQUIRED_FIELDS
                ? process.env.SERIAL_REQUIRED_FIELDS.split(',').map(f => f.trim()).filter(Boolean)
                : [],
            fieldsToEncrypt: process.env.SERIAL_FIELD_TO_ENCRYPT
                ? process.env.SERIAL_FIELD_TO_ENCRYPT.split(',').map(f => f.trim()).filter(Boolean)
                : [],
        };

        console.log('=== SerialManager Config Debug ===');
        console.log('Raw env delimiter:', JSON.stringify(rawDelimiter));
        console.log('Parsed delimiter:', JSON.stringify(parsedDelimiter));
        console.log('Raw env values:');
        console.log('  SERIAL_PORT:', process.env.SERIAL_PORT);
        console.log('  SERIAL_BAUDRATE:', process.env.SERIAL_BAUDRATE);
        console.log('  SERIAL_DATA_TYPES:', process.env.SERIAL_DATA_TYPES);
        console.log('  SERIAL_DB_TABLE_NAME:', process.env.SERIAL_DB_TABLE_NAME);
        console.log('Processed config:', JSON.stringify(config, null, 2));
        console.log('=== End Config Debug ===');

        return config;
    }

    async initialize() {
        try {
            this.serialCommunicator = new SerialCommunicator(
                this.config,
                this.database,
                this.mainWindow,
                this.csvLogger    // pass logger so _handleData can write lines
            );

            // Wait for window to load before connecting
            setTimeout(() => {
                console.log('Starting SerialCommunicator connection...');
                this.serialCommunicator.connect();
            }, 2000);

            console.log('Serial manager initialized');
        } catch (error) {
            console.error('Serial manager initialization failed:', error);
            throw error;
        }
    }

    getStatus() {
        return this.serialCommunicator ? this.serialCommunicator.getStatus() : null;
    }

    async forceReconnect() {
        if (this.serialCommunicator) {
            await this.serialCommunicator.forceReconnect();
        }
    }

    async disconnect() {
        if (this.serialCommunicator) {
            await this.serialCommunicator.disconnect();
        }
    }

    async scanForBetterPorts() {
        if (this.serialCommunicator) {
            await this.serialCommunicator.scanForBetterPorts();
        }
    }

    setDynamicPortSwitching(enabled) {
        if (this.serialCommunicator) {
            this.serialCommunicator.setDynamicPortSwitching(enabled);
        }
    }

    sendData(data) {
        if (this.serialCommunicator) {
            this.serialCommunicator.sendData(data);
        }
    }

    async close() {
        if (this.serialCommunicator) {
            try {
                await this.serialCommunicator.close();
                console.log('Serial communicator closed');
            } catch (error) {
                console.error('Error closing serial communicator:', error);
                throw error;
            }
        }
    }

    isConnected() {
        return this.serialCommunicator ? this.serialCommunicator.isConnected() : false;
    }
}

module.exports = SerialManager;