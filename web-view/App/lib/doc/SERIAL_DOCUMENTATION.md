# Serial Communication Handler Documentation

## Overview
The SerialCommunicator class provides a robust interface for serial communication with Arduino/ESP32 devices, featuring auto-reconnection, port scanning, and data management.

## Features
- Automatic port detection
- Dynamic port switching
- Auto-reconnection
- Data parsing and validation
- Database integration
- Real-time monitoring
- Error handling

## Configuration

```javascript
const config = {
    portPath: null,              // Serial port path (auto-detect if null)
    baudRate: 9600,             // Communication speed
    dataType: 'json-object',    // Data format
    lineDelimiter: '\r\n',      // Line ending
    csvDelimiter: ',',          // For CSV data
    fieldMapping: [],           // Data field mapping
    dbTableName: null,          // Database table for data storage
    requiredFields: [],         // Required data fields
    fieldsToEncrypt: [],        // Fields to encrypt
    autoReconnect: true,        // Enable auto-reconnection
    reconnectDelay: 3000,       // Delay between reconnection attempts
    maxReconnectAttempts: 10,   // Maximum reconnection attempts
    connectionTimeout: 5000,    // Connection timeout
    portScanInterval: 15000,    // Port scanning interval
    enableDynamicPortSwitching: true // Enable automatic port switching
};
```

## Basic Usage

### Initialization
```javascript
const serialComm = new SerialCommunicator(config, dbInstance, windowInstance);

// Connect to device
await serialComm.connect();

// Get connection status
const status = serialComm.getStatus();

// Send data to device
serialComm.sendData('command');

// Disconnect
await serialComm.disconnect();
```

## Advanced Features

### Auto Port Detection
```javascript
// Configure for auto-detection
const config = {
    portPath: null,  // Enable auto-detection
    baudRate: 9600
};

// The system will automatically scan for Arduino/ESP32 devices
await serialComm.connect();
```

### Dynamic Port Switching
```javascript
// Enable dynamic port switching
serialComm.setDynamicPortSwitching(true);

// Manually trigger port scanning
await serialComm.scanForBetterPorts();
```

### Reconnection Management
```javascript
// Configure reconnection
const config = {
    autoReconnect: true,
    reconnectDelay: 3000,
    maxReconnectAttempts: 10
};

// Force reconnection
await serialComm.forceReconnect();
```

## Data Handling

### JSON Data Format
```javascript
// Incoming data format
{
    "sensorId": "TEMP01",
    "value": 25.5,
    "timestamp": "2025-07-26T10:00:00Z"
}

// Configuration
const config = {
    dataType: 'json-object',
    requiredFields: ['sensorId', 'value', 'timestamp']
};
```

### CSV Data Format
```javascript
// Configuration for CSV data
const config = {
    dataType: 'csv',
    csvDelimiter: ',',
    fieldMapping: ['timestamp', 'sensorId', 'value']
};

// Example incoming data
// "2025-07-26 10:00:00,TEMP01,25.5"
```

## Database Integration

### Data Storage
```javascript
// Configure database connection
const config = {
    dbTableName: 'sensor_data',
    fieldsToEncrypt: ['sensorId']
};

// Data is automatically saved to database
_saveToDatabase(data) {
    let dataToInsert = { ...data };
    
    // Handle encryption if configured
    if (this.db.encrypt && this.config.fieldsToEncrypt) {
        this.config.fieldsToEncrypt.forEach(field => {
            if (dataToInsert[field]) {
                dataToInsert[field] = this.db.encrypt(dataToInsert[field]);
            }
        });
    }

    return this.db.postData(this.config.dbTableName, dataToInsert);
}
```

## Error Handling and Monitoring

### Connection States
```javascript
const connectionStates = {
    DISCONNECTED: 'disconnected',
    CONNECTING: 'connecting',
    CONNECTED: 'connected',
    RECONNECTING: 'reconnecting',
    SWITCHING_PORTS: 'switching_ports',
    ERROR: 'error'
};

// Monitor state changes
_setState(newState, message = '') {
    this.currentState = newState;
    this._sendToRenderer('serial-port-status', {
        state: newState,
        message: message,
        timestamp: new Date().toISOString()
    });
}
```

### Error Handling
```javascript
try {
    await serialComm.connect();
} catch (error) {
    console.error('Connection failed:', error);
    // Handle error appropriately
}

// Monitor for errors
serialComm.on('error', (error) => {
    console.error('Serial error:', error);
    // Implement error recovery
});
```

## Best Practices

1. **Enable Auto-Reconnection**
```javascript
const config = {
    autoReconnect: true,
    reconnectDelay: 3000,
    maxReconnectAttempts: 10
};
```

2. **Configure Data Validation**
```javascript
const config = {
    requiredFields: ['sensorId', 'value', 'timestamp'],
    dataType: 'json-object'
};
```

3. **Implement Error Recovery**
```javascript
serialComm.on('error', async (error) => {
    console.error('Serial error:', error);
    if (error.code === 'PORT_CLOSED') {
        await serialComm.forceReconnect();
    }
});
```

4. **Monitor Connection Health**
```javascript
setInterval(() => {
    const status = serialComm.getStatus();
    if (!status.isConnected) {
        // Handle disconnection
    }
}, 5000);
```

## Troubleshooting

### Common Issues

1. **Connection Failures**
   - Check physical connection
   - Verify port configuration
   - Check baudRate setting
   - Verify device drivers

2. **Data Reception Issues**
   - Check data format configuration
   - Verify line delimiters
   - Monitor data parsing errors

3. **Port Switching Problems**
   - Check port permissions
   - Verify device detection
   - Monitor port scanning logs

### Debugging Tips

1. **Enable Detailed Logging**
```javascript
// Monitor all data
serialComm.on('data', (data) => {
    console.log('Raw data:', data);
});

// Monitor state changes
serialComm.on('stateChange', (state) => {
    console.log('State changed:', state);
});
```

2. **Check Port Information**
```javascript
// Get detailed port info
const portInfo = serialComm.getPortInfo();
console.log('Port details:', portInfo);

// List available ports
SerialPort.list().then(ports => {
    console.log('Available ports:', ports);
});
```

3. **Monitor Connection Health**
```javascript
setInterval(() => {
    const status = serialComm.getStatus();
    console.log('Connection status:', status);
    console.log('Last data received:', new Date(status.lastDataReceived));
}, 10000);
```
