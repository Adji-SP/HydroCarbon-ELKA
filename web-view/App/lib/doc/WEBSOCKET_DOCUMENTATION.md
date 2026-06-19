# WebSocketHandler Library Documentation

## Table of Contents
1. [What is WebSocketHandler?](#what-is-websockethandler)
2. [Key Features](#key-features)
3. [Getting Started](#getting-started)
4. [Basic Setup](#basic-setup)
5. [Configuration Options](#configuration-options)
6. [Communication Patterns](#communication-patterns)
7. [Message Types](#message-types)
8. [Usage Examples](#usage-examples)
9. [Advanced Features](#advanced-features)
10. [Error Handling](#error-handling)
11. [Best Practices](#best-practices)

## What is WebSocketHandler?

WebSocketHandler is a powerful JavaScript library that creates a WebSocket server for real-time, two-way communication between applications. Think of it as a telephone system for your programs - it allows different devices, applications, or services to talk to each other instantly over the internet.

### Why Use WebSockets?
- **Real-time communication**: Messages are sent and received instantly
- **Two-way communication**: Both server and clients can initiate conversations
- **Efficient**: Low overhead compared to traditional HTTP requests
- **Persistent connection**: No need to reconnect for each message

## Key Features

### 🔄 Communication Patterns
- **Request-Response**: Ask a question and get an answer
- **Publish-Subscribe**: Broadcast messages to multiple subscribers
- **Remote Procedure Calls (RPC)**: Execute functions remotely
- **Data Streaming**: Send continuous data flows

### 🛡️ Security & Reliability
- **Authentication**: Token-based client verification
- **Heartbeat monitoring**: Detect and handle dead connections
- **Data validation**: Ensure incoming data meets requirements
- **Encryption support**: Protect sensitive data

### 📊 Data Management
- **Database integration**: Automatically save data to databases
- **Multiple client types**: Support for microcontrollers, applications, and services
- **Binary data support**: Handle files, images, and other binary content
- **Message compression**: Reduce bandwidth usage

## Getting Started

### Prerequisites
- Node.js installed on your system
- Basic understanding of JavaScript
- A database instance (optional, for data storage)

### Installation
```javascript
const WebSocketHandler = require('./WebSocketHandler');
```

## Basic Setup

### Minimal Configuration
```javascript
// Create a simple WebSocket server
const wsHandler = new WebSocketHandler({
    port: 8080,
    host: '0.0.0.0'
});

// Start the server
wsHandler.start()
    .then(() => {
        console.log('WebSocket server is running!');
    })
    .catch(error => {
        console.error('Failed to start server:', error);
    });
```

### With Database Integration
```javascript
const WebSocketHandler = require('./WebSocketHandler');
const DatabaseHandler = require('./DatabaseHandler'); // Your database handler

// Initialize database
const db = new DatabaseHandler(/* database config */);

// Create WebSocket server with database
const wsHandler = new WebSocketHandler({
    port: 8080,
    enableAuthentication: true,
    dbTableName: 'sensor_data',
    requiredFields: ['temperature', 'humidity']
}, db);

// Start server
wsHandler.start();
```

## Configuration Options

### Basic Options
```javascript
const config = {
    // Server settings
    port: 8080,                    // Port to listen on
    host: '0.0.0.0',              // Host address
    maxConnections: 100,           // Maximum concurrent connections
    
    // Authentication
    enableAuthentication: false,   // Require client authentication
    authToken: 'your-secret-token', // Authentication token
    
    // Features
    enableRequestResponse: true,   // Enable request-response pattern
    enablePubSub: true,           // Enable publish-subscribe
    enableRPC: true,              // Enable remote procedure calls
    enableStreaming: true,        // Enable data streaming
    
    // Data handling
    enableDataValidation: true,    // Validate incoming data
    requiredFields: [],           // Fields that must be present
    fieldsToEncrypt: [],          // Fields to encrypt before database storage
    
    // Performance
    enableCompression: true,       // Compress messages
    maxPayloadSize: 10485760,     // Max message size (10MB)
    requestTimeout: 10000,        // Request timeout (10 seconds)
    
    // Monitoring
    enableHeartbeat: true,         // Monitor connection health
    heartbeatInterval: 30000,      // Heartbeat interval (30 seconds)
    logLevel: 'info'              // Logging level (debug, info, warn, error)
};
```

### Client Types
The library supports different types of clients:
- **microcontroller**: Arduino, Raspberry Pi, IoT devices
- **application**: Web apps, mobile apps, desktop applications
- **service**: Backend services, APIs, microservices

## Communication Patterns

### 1. Simple Data Exchange
**Client sends data to server:**
```javascript
// Client sends sensor data
{
    type: 'data',
    dataType: 'sensor',
    data: {
        temperature: 25.5,
        humidity: 60,
        location: 'bedroom'
    }
}

// Server responds
{
    type: 'data_response',
    success: true,
    timestamp: '2024-01-01T12:00:00.000Z'
}
```

### 2. Request-Response Pattern
**Client asks server to perform an action:**
```javascript
// Client request
{
    type: 'request',
    requestId: 'req_123',
    endpoint: 'get_weather',
    data: { city: 'New York' }
}

// Server response
{
    type: 'response',
    requestId: 'req_123',
    success: true,
    data: {
        temperature: 22,
        condition: 'sunny'
    }
}
```

### 3. Publish-Subscribe Pattern
**Broadcasting messages to multiple subscribers:**
```javascript
// Client subscribes to temperature updates
{
    type: 'subscribe',
    topic: 'temperature-alerts'
}

// Server publishes to all subscribers
{
    type: 'publish',
    topic: 'temperature-alerts',
    data: {
        message: 'High temperature detected!',
        value: 35,
        location: 'server-room'
    }
}
```

### 4. Remote Procedure Calls (RPC)
**Execute functions on the server:**
```javascript
// Client calls server function
{
    type: 'rpc_call',
    requestId: 'rpc_456',
    method: 'calculateAverage',
    params: {
        values: [10, 20, 30, 40, 50]
    }
}

// Server returns result
{
    type: 'rpc_response',
    requestId: 'rpc_456',
    success: true,
    data: 30
}
```

## Message Types

### Core Message Types

#### Data Messages
```javascript
// Send general data
{
    type: 'data',
    dataType: 'sensor',  // or 'application', 'service'
    data: { /* your data */ }
}

// Legacy sensor data (for backward compatibility)
{
    type: 'sensor_data',
    temperature: 25.0,
    humidity: 60
}
```

#### Authentication
```javascript
{
    type: 'auth',
    token: 'your-auth-token'
}
```

#### Heartbeat
```javascript
{
    type: 'heartbeat'
}
// Server responds with heartbeat_response
```

### Advanced Message Types

#### Streaming
```javascript
// Start a stream
{
    type: 'stream_start',
    streamId: 'video_stream_1',
    streamType: 'video'
}

// Send stream data
{
    type: 'stream_data',
    streamId: 'video_stream_1',
    data: /* binary or encoded data */,
    sequenceNumber: 1
}

// End stream
{
    type: 'stream_end',
    streamId: 'video_stream_1'
}
```

## Usage Examples

### Example 1: IoT Temperature Sensor
```javascript
// Server setup
const wsHandler = new WebSocketHandler({
    port: 8080,
    enableAuthentication: true,
    authToken: 'sensor-secret-123',
    dbTableName: 'temperature_readings',
    requiredFields: ['temperature', 'sensor_id'],
    enableDataValidation: true
}, databaseInstance);

// Handle incoming sensor data
wsHandler.on('data-received', (event) => {
    if (event.dataType === 'sensor') {
        console.log(`Temperature: ${event.data.temperature}°C from sensor ${event.data.sensor_id}`);
        
        // Check for alerts
        if (event.data.temperature > 30) {
            wsHandler.publish('temperature-alerts', {
                message: 'High temperature detected!',
                temperature: event.data.temperature,
                sensor: event.data.sensor_id
            });
        }
    }
});

wsHandler.start();
```

### Example 2: Chat Application
```javascript
// Setup for chat application
const chatServer = new WebSocketHandler({
    port: 3000,
    enablePubSub: true,
    enableAuthentication: false
});

// Handle new messages
chatServer.on('data-received', (event) => {
    if (event.dataType === 'chat') {
        // Broadcast message to all chat participants
        chatServer.publish('chat-room', {
            user: event.data.username,
            message: event.data.message,
            timestamp: new Date().toISOString()
        });
    }
});

chatServer.start();
```

### Example 3: Remote Control System
```javascript
// Setup RPC methods for device control
const controlServer = new WebSocketHandler({
    port: 4000,
    enableRPC: true,
    enableAuthentication: true
});

// Register device control methods
controlServer.registerRPCMethod('turnOnLight', (params) => {
    const { roomId, brightness } = params;
    // Control smart light
    return controlSmartLight(roomId, true, brightness);
});

controlServer.registerRPCMethod('getTemperature', (params) => {
    const { roomId } = params;
    // Get temperature from sensor
    return getCurrentTemperature(roomId);
});

controlServer.start();
```

## Advanced Features

### Client Management
```javascript
// Get server status and connected clients
const status = wsHandler.getStatus();
console.log(`Connected clients: ${status.connectionCount}`);
console.log('Client details:', status.clients);

// Send message to specific client
wsHandler.sendToClient('client_123', {
    message: 'Hello specific client!'
});

// Broadcast to all clients of specific type
wsHandler.broadcastToAll({
    message: 'System maintenance in 5 minutes'
}, 'microcontroller'); // Only send to microcontroller clients
```

### Stream Management
```javascript
// Server-initiated streaming
wsHandler.startStream('client_456', 'data_stream_1', 'sensor_data');

// Send stream data
setInterval(() => {
    wsHandler.sendStreamData('data_stream_1', {
        timestamp: new Date().toISOString(),
        value: Math.random() * 100
    });
}, 1000);

// End stream when done
setTimeout(() => {
    wsHandler.endStream('data_stream_1');
}, 60000);
```

### Event Handling
```javascript
// Listen to various events
wsHandler.on('client-connected', (event) => {
    console.log(`New client connected: ${event.clientId}`);
});

wsHandler.on('client-disconnected', (event) => {
    console.log(`Client disconnected: ${event.clientId}, Duration: ${event.connectedDuration}ms`);
});

wsHandler.on('database-insert', (event) => {
    console.log(`Data saved to database with ID: ${event.insertId}`);
});

wsHandler.on('stream-started', (event) => {
    console.log(`Stream started: ${event.streamId} by ${event.clientId}`);
});
```

## Error Handling

### Server-side Error Handling
```javascript
wsHandler.on('server-error', (error) => {
    console.error('Server error:', error.message);
    // Implement recovery logic
});

wsHandler.on('client-error', (event) => {
    console.error(`Client ${event.clientId} error:`, event.error.message);
});

wsHandler.on('database-error', (event) => {
    console.error(`Database error for client ${event.clientId}:`, event.error.message);
    // Maybe retry or use backup storage
});
```

### Graceful Shutdown
```javascript
// Handle process termination
process.on('SIGINT', async () => {
    console.log('Shutting down gracefully...');
    try {
        await wsHandler.stop();
        console.log('WebSocket server stopped');
        process.exit(0);
    } catch (error) {
        console.error('Error during shutdown:', error);
        process.exit(1);
    }
});
```

## Best Practices

### 1. Security
- Always use authentication in production
- Keep auth tokens secret and rotate them regularly
- Validate all incoming data
- Use HTTPS/WSS in production environments

```javascript
const secureConfig = {
    enableAuthentication: true,
    authToken: process.env.WEBSOCKET_TOKEN, // Use environment variables
    enableDataValidation: true,
    requiredFields: ['timestamp', 'source'],
    fieldsToEncrypt: ['sensitive_data']
};
```

### 2. Performance
- Set appropriate connection limits
- Enable compression for large messages
- Use heartbeat to detect dead connections
- Implement proper logging levels

```javascript
const performanceConfig = {
    maxConnections: 1000,
    enableCompression: true,
    heartbeatInterval: 30000,
    maxPayloadSize: 1024 * 1024, // 1MB
    logLevel: 'warn' // Reduce log noise in production
};
```

### 3. Reliability
- Implement reconnection logic on client side
- Handle network interruptions gracefully
- Use request timeouts to avoid hanging requests
- Monitor connection health

```javascript
// Client-side reconnection example (pseudo-code)
function connectWithRetry() {
    const ws = new WebSocket('ws://localhost:8080');
    
    ws.onclose = (event) => {
        console.log('Connection lost, retrying in 5 seconds...');
        setTimeout(connectWithRetry, 5000);
    };
}
```

### 4. Data Management
- Structure your data consistently
- Use appropriate data types for different clients
- Implement data validation rules
- Consider data encryption for sensitive information

```javascript
const dataConfig = {
    clientTypes: ['sensor', 'dashboard', 'mobile_app'],
    requiredFields: ['timestamp', 'client_type', 'data'],
    enableDataValidation: true,
    fieldsToEncrypt: ['user_data', 'location']
};
```

### 5. Monitoring and Debugging
- Use appropriate log levels
- Monitor connection counts and performance
- Track message patterns and errors
- Implement health checks

```javascript
// Regular status monitoring
setInterval(() => {
    const status = wsHandler.getStatus();
    console.log(`Status - Connections: ${status.connectionCount}, Uptime: ${status.uptime}ms`);
    
    if (status.connectionCount > status.maxConnections * 0.9) {
        console.warn('Approaching connection limit!');
    }
}, 60000); // Check every minute
```

## Client Implementation Examples

### 1. Arduino/ESP32 Microcontroller Implementation

#### Basic ESP32 WebSocket Client
```cpp
#include <WiFi.h>
#include <WebSocketsClient.h>
#include <ArduinoJson.h>
#include <DHT.h>

// WiFi credentials
const char* ssid = "your-wifi-ssid";
const char* password = "your-wifi-password";

// WebSocket server details
const char* websocket_server = "192.168.1.100";
const int websocket_port = 8080;
const char* auth_token = "sensor-secret-123";

// Sensor setup
#define DHT_PIN 2
#define DHT_TYPE DHT22
DHT dht(DHT_PIN, DHT_TYPE);

WebSocketsClient webSocket;
String clientId = "";
bool isAuthenticated = false;
bool isConnected = false;

// Timing variables
unsigned long lastSensorRead = 0;
unsigned long lastHeartbeat = 0;
const unsigned long SENSOR_INTERVAL = 10000; // 10 seconds
const unsigned long HEARTBEAT_INTERVAL = 30000; // 30 seconds

void setup() {
    Serial.begin(115200);
    dht.begin();
    
    // Connect to WiFi
    WiFi.begin(ssid, password);
    while (WiFi.status() != WL_CONNECTED) {
        delay(1000);
        Serial.println("Connecting to WiFi...");
    }
    Serial.println("WiFi connected!");
    Serial.print("IP address: ");
    Serial.println(WiFi.localIP());
    
    // Setup WebSocket connection
    webSocket.begin(websocket_server, websocket_port, "/");
    webSocket.onEvent(webSocketEvent);
    webSocket.setReconnectInterval(5000);
    
    Serial.println("WebSocket client started");
}

void loop() {
    webSocket.loop();
    
    if (isConnected && isAuthenticated) {
        unsigned long currentTime = millis();
        
        // Send sensor data periodically
        if (currentTime - lastSensorRead >= SENSOR_INTERVAL) {
            sendSensorData();
            lastSensorRead = currentTime;
        }
        
        // Send heartbeat periodically
        if (currentTime - lastHeartbeat >= HEARTBEAT_INTERVAL) {
            sendHeartbeat();
            lastHeartbeat = currentTime;
        }
    }
    
    delay(100);
}

void webSocketEvent(WStype_t type, uint8_t * payload, size_t length) {
    switch(type) {
        case WStype_DISCONNECTED:
            Serial.println("WebSocket Disconnected");
            isConnected = false;
            isAuthenticated = false;
            clientId = "";
            break;
            
        case WStype_CONNECTED:
            Serial.printf("WebSocket Connected to: %s\n", payload);
            isConnected = true;
            break;
            
        case WStype_TEXT:
            handleMessage((char*)payload);
            break;
            
        case WStype_ERROR:
            Serial.printf("WebSocket Error: %s\n", payload);
            break;
            
        default:
            break;
    }
}

void handleMessage(const char* message) {
    DynamicJsonDocument doc(1024);
    DeserializationError error = deserializeJson(doc, message);
    
    if (error) {
        Serial.print("JSON parsing failed: ");
        Serial.println(error.c_str());
        return;
    }
    
    String messageType = doc["type"];
    
    if (messageType == "welcome") {
        clientId = doc["clientId"].as<String>();
        Serial.println("Received welcome message");
        Serial.println("Client ID: " + clientId);
        
        // Send handshake
        sendHandshake();
        
        // Authenticate if required
        if (doc["authRequired"]) {
            authenticate();
        } else {
            isAuthenticated = true;
        }
    }
    else if (messageType == "handshake_response") {
        Serial.println("Handshake successful");
    }
    else if (messageType == "auth_response") {
        bool success = doc["success"];
        if (success) {
            Serial.println("Authentication successful");
            isAuthenticated = true;
        } else {
            Serial.println("Authentication failed");
        }
    }
    else if (messageType == "data_response") {
        bool success = doc["success"];
        if (success) {
            Serial.println("Data sent successfully");
            if (doc.containsKey("insertId")) {
                Serial.println("Database ID: " + String(doc["insertId"].as<int>()));
            }
        } else {
            Serial.println("Data sending failed: " + String(doc["message"].as<String>()));
        }
    }
    else if (messageType == "request") {
        // Handle incoming requests from server
        handleServerRequest(doc);
    }
    else if (messageType == "publish") {
        // Handle published messages
        handlePublishedMessage(doc);
    }
}

void sendHandshake() {
    DynamicJsonDocument doc(512);
    doc["type"] = "handshake";
    doc["clientType"] = "microcontroller";
    JsonArray capabilities = doc.createNestedArray("capabilities");
    capabilities.add("sensor_data");
    capabilities.add("remote_control");
    doc["version"] = "1.0.0";
    
    String message;
    serializeJson(doc, message);
    webSocket.sendTXT(message);
}

void authenticate() {
    DynamicJsonDocument doc(256);
    doc["type"] = "auth";
    doc["token"] = auth_token;
    
    String message;
    serializeJson(doc, message);
    webSocket.sendTXT(message);
}

void sendSensorData() {
    float temperature = dht.readTemperature();
    float humidity = dht.readHumidity();
    
    if (isnan(temperature) || isnan(humidity)) {
        Serial.println("Failed to read from DHT sensor!");
        return;
    }
    
    DynamicJsonDocument doc(512);
    doc["type"] = "sensor_data";
    doc["temperature"] = temperature;
    doc["humidity"] = humidity;
    doc["sensor_id"] = "ESP32_DHT22_01";
    doc["location"] = "living_room";
    doc["battery_level"] = 85; // Mock battery level
    doc["timestamp"] = millis();
    
    String message;
    serializeJson(doc, message);
    webSocket.sendTXT(message);
    
    Serial.printf("Sent: T=%.1f°C, H=%.1f%%\n", temperature, humidity);
}

void sendHeartbeat() {
    DynamicJsonDocument doc(128);
    doc["type"] = "heartbeat";
    doc["timestamp"] = millis();
    
    String message;
    serializeJson(doc, message);
    webSocket.sendTXT(message);
    
    Serial.println("Heartbeat sent");
}

void handleServerRequest(const DynamicJsonDocument& doc) {
    String endpoint = doc["endpoint"];
    String requestId = doc["requestId"];
    
    DynamicJsonDocument response(512);
    response["type"] = "response";
    response["requestId"] = requestId;
    
    if (endpoint == "get_sensor_status") {
        response["success"] = true;
        response["data"]["temperature"] = dht.readTemperature();
        response["data"]["humidity"] = dht.readHumidity();
        response["data"]["uptime"] = millis();
        response["data"]["free_heap"] = ESP.getFreeHeap();
    }
    else if (endpoint == "reboot") {
        response["success"] = true;
        response["data"]["message"] = "Rebooting in 3 seconds";
        
        String message;
        serializeJson(response, message);
        webSocket.sendTXT(message);
        
        delay(3000);
        ESP.restart();
        return;
    }
    else {
        response["success"] = false;
        response["error"] = "Unknown endpoint: " + endpoint;
    }
    
    String message;
    serializeJson(response, message);
    webSocket.sendTXT(message);
}

void handlePublishedMessage(const DynamicJsonDocument& doc) {
    String topic = doc["topic"];
    
    if (topic == "system_alerts") {
        Serial.println("System Alert: " + String(doc["data"]["message"].as<String>()));
        // Handle system alerts (maybe blink LED, etc.)
    }
}
```

### 2. Python Application Implementation

#### Simple Python WebSocket Client
```python
import asyncio
import websockets
import json
import time
import logging
from datetime import datetime
import random

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class WebSocketClient:
    def __init__(self, uri, auth_token=None):
        self.uri = uri
        self.auth_token = auth_token
        self.websocket = None
        self.client_id = None
        self.is_authenticated = False
        self.is_connected = False
        self.pending_requests = {}
        
    async def connect(self):
        """Connect to WebSocket server"""
        try:
            self.websocket = await websockets.connect(self.uri)
            self.is_connected = True
            logger.info(f"Connected to {self.uri}")
            
            # Start message handler
            asyncio.create_task(self.message_handler())
            
        except Exception as e:
            logger.error(f"Connection failed: {e}")
            raise
    
    async def disconnect(self):
        """Disconnect from WebSocket server"""
        if self.websocket:
            await self.websocket.close()
            self.is_connected = False
            logger.info("Disconnected from server")
    
    async def send_message(self, message):
        """Send message to server"""
        if self.websocket and self.is_connected:
            await self.websocket.send(json.dumps(message))
        else:
            logger.error("Not connected to server")
    
    async def message_handler(self):
        """Handle incoming messages"""
        try:
            async for message in self.websocket:
                try:
                    data = json.loads(message)
                    await self.handle_message(data)
                except json.JSONDecodeError as e:
                    logger.error(f"JSON decode error: {e}")
                except Exception as e:
                    logger.error(f"Message handling error: {e}")
        except websockets.exceptions.ConnectionClosed:
            logger.info("Connection closed by server")
            self.is_connected = False
    
    async def handle_message(self, data):
        """Handle different message types"""
        message_type = data.get('type')
        
        if message_type == 'welcome':
            self.client_id = data.get('clientId')
            logger.info(f"Received welcome, client ID: {self.client_id}")
            
            # Send handshake
            await self.send_handshake()
            
            # Authenticate if required
            if data.get('authRequired') and self.auth_token:
                await self.authenticate()
            else:
                self.is_authenticated = True
        
        elif message_type == 'handshake_response':
            logger.info("Handshake successful")
        
        elif message_type == 'auth_response':
            success = data.get('success')
            if success:
                logger.info("Authentication successful")
                self.is_authenticated = True
            else:
                logger.error("Authentication failed")
        
        elif message_type == 'data_response':
            success = data.get('success')
            if success:
                logger.info("Data sent successfully")
                if 'insertId' in data:
                    logger.info(f"Database ID: {data['insertId']}")
            else:
                logger.error(f"Data sending failed: {data.get('message')}")
        
        elif message_type == 'request':
            await self.handle_server_request(data)
        
        elif message_type == 'response':
            await self.handle_response(data)
        
        elif message_type == 'publish':
            await self.handle_published_message(data)
        
        elif message_type == 'error':
            logger.error(f"Server error: {data.get('message')}")
    
    async def send_handshake(self):
        """Send handshake message"""
        handshake = {
            'type': 'handshake',
            'clientType': 'application',
            'capabilities': ['data_processing', 'file_upload', 'analytics'],
            'version': '1.0.0'
        }
        await self.send_message(handshake)
    
    async def authenticate(self):
        """Send authentication message"""
        auth_msg = {
            'type': 'auth',
            'token': self.auth_token
        }
        await self.send_message(auth_msg)
    
    async def send_data(self, data, data_type='application'):
        """Send data to server"""
        message = {
            'type': 'data',
            'dataType': data_type,
            'data': data,
            'timestamp': datetime.now().isoformat()
        }
        await self.send_message(message)
    
    async def send_request(self, endpoint, data=None, timeout=10):
        """Send request and wait for response"""
        request_id = f"req_{int(time.time() * 1000)}_{random.randint(1000, 9999)}"
        
        request = {
            'type': 'request',
            'requestId': request_id,
            'endpoint': endpoint,
            'data': data or {}
        }
        
        # Create future for response
        future = asyncio.Future()
        self.pending_requests[request_id] = future
        
        # Send request
        await self.send_message(request)
        
        # Wait for response with timeout
        try:
            response = await asyncio.wait_for(future, timeout=timeout)
            return response
        except asyncio.TimeoutError:
            self.pending_requests.pop(request_id, None)
            raise TimeoutError(f"Request {endpoint} timed out")
    
    async def subscribe(self, topic):
        """Subscribe to a topic"""
        message = {
            'type': 'subscribe',
            'topic': topic
        }
        await self.send_message(message)
        logger.info(f"Subscribed to topic: {topic}")
    
    async def publish(self, topic, data):
        """Publish to a topic"""
        message = {
            'type': 'publish',
            'topic': topic,
            'data': data
        }
        await self.send_message(message)
        logger.info(f"Published to topic: {topic}")
    
    async def handle_server_request(self, data):
        """Handle requests from server"""
        endpoint = data.get('endpoint')
        request_id = data.get('requestId')
        request_data = data.get('data', {})
        
        logger.info(f"Received server request: {endpoint}")
        
        response = {
            'type': 'response',
            'requestId': request_id,
            'success': True,
            'data': {}
        }
        
        try:
            if endpoint == 'get_system_info':
                import platform
                import psutil
                
                response['data'] = {
                    'platform': platform.platform(),
                    'cpu_percent': psutil.cpu_percent(),
                    'memory_percent': psutil.virtual_memory().percent,
                    'disk_usage': psutil.disk_usage('/').percent
                }
            
            elif endpoint == 'process_data':
                # Example data processing
                input_data = request_data.get('values', [])
                result = sum(input_data) / len(input_data) if input_data else 0
                response['data'] = {'average': result}
            
            else:
                response['success'] = False
                response['error'] = f"Unknown endpoint: {endpoint}"
        
        except Exception as e:
            response['success'] = False
            response['error'] = str(e)
        
        await self.send_message(response)
    
    async def handle_response(self, data):
        """Handle response to our requests"""
        request_id = data.get('requestId')
        if request_id in self.pending_requests:
            future = self.pending_requests.pop(request_id)
            if not future.cancelled():
                if data.get('success'):
                    future.set_result(data.get('data'))
                else:
                    future.set_exception(Exception(data.get('error', 'Request failed')))
    
    async def handle_published_message(self, data):
        """Handle published messages"""
        topic = data.get('topic')
        message_data = data.get('data')
        
        logger.info(f"Received published message on topic '{topic}': {message_data}")
        
        # Handle different topics
        if topic == 'system_alerts':
            logger.warning(f"System Alert: {message_data.get('message')}")
        elif topic == 'sensor_updates':
            logger.info(f"Sensor Update: {message_data}")

# Example usage
async def main():
    # Create client
    client = WebSocketClient('ws://localhost:8080', 'your-auth-token')
    
    try:
        # Connect to server
        await client.connect()
        
        # Wait for authentication
        while not client.is_authenticated:
            await asyncio.sleep(0.1)
        
        # Subscribe to topics
        await client.subscribe('system_alerts')
        await client.subscribe('sensor_updates')
        
        # Send some sample data
        await client.send_data({
            'application': 'data_processor',
            'version': '1.0.0',
            'processed_records': 150,
            'processing_time': 2.5,
            'status': 'healthy'
        })
        
        # Send a request
        try:
            response = await client.send_request('get_weather', {'city': 'New York'})
            logger.info(f"Weather response: {response}")
        except Exception as e:
            logger.error(f"Request failed: {e}")
        
        # Publish a message
        await client.publish('data_updates', {
            'source': 'python_client',
            'message': 'Processing completed successfully',
            'timestamp': datetime.now().isoformat()
        })
        
        # Keep connection alive
        while client.is_connected:
            await asyncio.sleep(1)
            
            # Send periodic data
            await client.send_data({
                'timestamp': datetime.now().isoformat(),
                'random_value': random.randint(1, 100),
                'status': 'running'
            })
            
            await asyncio.sleep(10)  # Wait 10 seconds
    
    except KeyboardInterrupt:
        logger.info("Shutting down...")
    finally:
        await client.disconnect()

if __name__ == '__main__':
    asyncio.run(main())
```

#### Advanced Python Client with File Upload
```python
import asyncio
import websockets
import json
import base64
import os
from pathlib import Path

class FileUploadClient(WebSocketClient):
    async def upload_file(self, file_path, chunk_size=8192):
        """Upload file in chunks"""
        file_path = Path(file_path)
        if not file_path.exists():
            raise FileNotFoundError(f"File not found: {file_path}")
        
        file_size = file_path.stat().st_size
        stream_id = f"upload_{int(time.time())}_{file_path.name}"
        
        # Start file upload stream
        await self.send_message({
            'type': 'stream_start',
            'streamId': stream_id,
            'streamType': 'file_upload',
            'metadata': {
                'filename': file_path.name,
                'size': file_size,
                'mime_type': 'application/octet-stream'
            }
        })
        
        # Send file in chunks
        sequence = 0
        with open(file_path, 'rb') as f:
            while True:
                chunk = f.read(chunk_size)
                if not chunk:
                    break
                
                # Encode chunk to base64
                encoded_chunk = base64.b64encode(chunk).decode('utf-8')
                
                await self.send_message({
                    'type': 'stream_data',
                    'streamId': stream_id,
                    'data': encoded_chunk,
                    'sequenceNumber': sequence
                })
                
                sequence += 1
                logger.info(f"Sent chunk {sequence}, {len(chunk)} bytes")
        
        # End stream
        await self.send_message({
            'type': 'stream_end',
            'streamId': stream_id
        })
        
        logger.info(f"File upload completed: {file_path.name}")
```

### 3. Docker Implementation

#### Dockerfile for Python Client
```dockerfile
# Dockerfile for Python WebSocket Client
FROM python:3.9-slim

WORKDIR /app

# Install required packages
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY websocket_client.py .
COPY config.json .

# Create non-root user
RUN useradd -m -u 1000 wsuser && chown -R wsuser:wsuser /app
USER wsuser

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD python -c "import socket; socket.create_connection(('localhost', 8080), timeout=5)"

CMD ["python", "websocket_client.py"]
```

#### requirements.txt
```txt
websockets==11.0.3
aiofiles==23.1.0
psutil==5.9.5
python-dotenv==1.0.0
```

#### Docker Compose for Complete Setup
```yaml
# docker-compose.yml
version: '3.8'

services:
  # WebSocket Server
  websocket-server:
    build:
      context: ./server
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
    environment:
      - NODE_ENV=production
      - DB_HOST=database
      - DB_USER=root
      - DB_PASSWORD=password
      - DB_NAME=websocket_db
      - WS_AUTH_TOKEN=your-secret-token
    depends_on:
      - database
    restart: unless-stopped
    networks:
      - websocket-network

  # Database
  database:
    image: mysql:8.0
    environment:
      - MYSQL_ROOT_PASSWORD=password
      - MYSQL_DATABASE=websocket_db
    volumes:
      - db_data:/var/lib/mysql
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
    restart: unless-stopped
    networks:
      - websocket-network

  # Python Data Processor
  data-processor:
    build:
      context: ./clients/python
      dockerfile: Dockerfile
    environment:
      - WEBSOCKET_URL=ws://websocket-server:8080
      - AUTH_TOKEN=your-secret-token
      - CLIENT_TYPE=service
    depends_on:
      - websocket-server
    restart: unless-stopped
    networks:
      - websocket-network

  # Python Analytics Service
  analytics-service:
    build:
      context: ./clients/python
      dockerfile: Dockerfile.analytics
    environment:
      - WEBSOCKET_URL=ws://websocket-server:8080
      - AUTH_TOKEN=your-secret-token
      - REDIS_URL=redis://redis:6379
    depends_on:
      - websocket-server
      - redis
    restart: unless-stopped
    networks:
      - websocket-network

  # Redis for caching
  redis:
    image: redis:7-alpine
    restart: unless-stopped
    networks:
      - websocket-network

  # Nginx Load Balancer (for scaling)
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - websocket-server
    restart: unless-stopped
    networks:
      - websocket-network

volumes:
  db_data:

networks:
  websocket-network:
    driver: bridge
```

#### Nginx Configuration for WebSocket
```nginx
# nginx.conf
events {
    worker_connections 1024;
}

http {
    upstream websocket_backend {
        server websocket-server:8080;
        # Add more servers for load balancing
        # server websocket-server-2:8080;
    }

    map $http_upgrade $connection_upgrade {
        default upgrade;
        '' close;
    }

    server {
        listen 80;
        server_name your-domain.com;

        location / {
            proxy_pass http://websocket_backend;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection $connection_upgrade;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            
            # WebSocket specific settings
            proxy_read_timeout 86400;
            proxy_send_timeout 86400;
        }
    }
}
```

### 4. React.js Web Application Example

#### React WebSocket Hook
```javascript
// useWebSocket.js
import { useState, useEffect, useRef, useCallback } from 'react';

const useWebSocket = (url, authToken = null) => {
    const [socket, setSocket] = useState(null);
    const [isConnected, setIsConnected] = useState(false);
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [messages, setMessages] = useState([]);
    const [clientId, setClientId] = useState(null);
    const reconnectTimeoutRef = useRef(null);

    const connect = useCallback(() => {
        try {
            const ws = new WebSocket(url);
            
            ws.onopen = () => {
                console.log('WebSocket connected');
                setIsConnected(true);
                setSocket(ws);
            };

            ws.onmessage = (event) => {
                const data = JSON.parse(event.data);
                handleMessage(data);
            };

            ws.onclose = () => {
                console.log('WebSocket disconnected');
                setIsConnected(false);
                setIsAuthenticated(false);
                setSocket(null);
                
                // Attempt reconnection after 5 seconds
                reconnectTimeoutRef.current = setTimeout(() => {
                    connect();
                }, 5000);
            };

            ws.onerror = (error) => {
                console.error('WebSocket error:', error);
            };

        } catch (error) {
            console.error('Connection failed:', error);
        }
    }, [url]);

    const handleMessage = (data) => {
        setMessages(prev => [...prev, data]);
        
        switch (data.type) {
            case 'welcome':
                setClientId(data.clientId);
                
                // Send handshake
                sendMessage({
                    type: 'handshake',
                    clientType: 'application',
                    capabilities: ['real_time_display', 'user_interface'],
                    version: '1.0.0'
                });
                
                // Authenticate if required
                if (data.authRequired && authToken) {
                    sendMessage({
                        type: 'auth',
                        token: authToken
                    });
                } else if (!data.authRequired) {
                    setIsAuthenticated(true);
                }
                break;
                
            case 'auth_response':
                if (data.success) {
                    setIsAuthenticated(true);
                }
                break;
                
            default:
                break;
        }
    };

    const sendMessage = useCallback((message) => {
        if (socket && socket.readyState === WebSocket.OPEN) {
            socket.send(JSON.stringify(message));
        }
    }, [socket]);

    const sendData = useCallback((data, dataType = 'application') => {
        sendMessage({
            type: 'data',
            dataType,
            data,
            timestamp: new Date().toISOString()
        });
    }, [sendMessage]);

    const subscribe = useCallback((topic) => {
        sendMessage({
            type: 'subscribe',
            topic
        });
    }, [sendMessage]);

    const publish = useCallback((topic, data) => {
        sendMessage({
            type: 'publish',
            topic,
            data
        });
    }, [sendMessage]);

    useEffect(() => {
        connect();
        
        return (
        <div className="dashboard">
            <header className="dashboard-header">
                <h1>WebSocket Dashboard</h1>
                <div className="connection-status">
                    <span className={`status-indicator ${isConnected ? 'connected' : 'disconnected'}`}>
                        {isConnected ? '🟢' : '🔴'}
                    </span>
                    <span>
                        {isConnected 
                            ? (isAuthenticated ? `Connected (${clientId})` : 'Authenticating...') 
                            : 'Disconnected'
                        }
                    </span>
                </div>
            </header>

            <div className="dashboard-grid">
                {/* Sensor Data Section */}
                <div className="dashboard-card">
                    <h2>Live Sensor Data</h2>
                    <div className="sensor-grid">
                        {sensorData.slice(-6).map((data, index) => (
                            <div key={index} className="sensor-item">
                                <h3>{data.sensor_id || 'Unknown Sensor'}</h3>
                                <div className="sensor-values">
                                    {data.temperature && (
                                        <div className="sensor-value">
                                            <span className="label">Temp:</span>
                                            <span className="value">{data.temperature}°C</span>
                                        </div>
                                    )}
                                    {data.humidity && (
                                        <div className="sensor-value">
                                            <span className="label">Humidity:</span>
                                            <span className="value">{data.humidity}%</span>
                                        </div>
                                    )}
                                </div>
                                <small className="timestamp">
                                    {new Date(data.timestamp).toLocaleTimeString()}
                                </small>
                            </div>
                        ))}
                    </div>
                </div>

                {/* Alerts Section */}
                <div className="dashboard-card">
                    <h2>System Alerts</h2>
                    <div className="alerts-list">
                        {alerts.slice(-10).map((alert, index) => (
                            <div key={index} className={`alert-item ${alert.level || 'info'}`}>
                                <div className="alert-message">{alert.message}</div>
                                <small className="alert-time">
                                    {new Date(alert.timestamp).toLocaleTimeString()}
                                </small>
                            </div>
                        ))}
                    </div>
                </div>

                {/* Controls Section */}
                <div className="dashboard-card">
                    <h2>Controls</h2>
                    <div className="controls">
                        <button 
                            onClick={handleSendTestData}
                            disabled={!isAuthenticated}
                            className="btn btn-primary"
                        >
                            Send Test Data
                        </button>
                        <button 
                            onClick={handleSendAlert}
                            disabled={!isAuthenticated}
                            className="btn btn-secondary"
                        >
                            Send Test Alert
                        </button>
                    </div>
                </div>

                {/* Message Log */}
                <div className="dashboard-card full-width">
                    <h2>Message Log</h2>
                    <div className="message-log">
                        {messages.slice(-20).map((msg, index) => (
                            <div key={index} className="log-entry">
                                <span className="log-type">[{msg.type}]</span>
                                <span className="log-content">
                                    {JSON.stringify(msg, null, 2)}
                                </span>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );) => {
            if (reconnectTimeoutRef.current) {
                clearTimeout(reconnectTimeoutRef.current);
            }
            if (socket) {
                socket.close();
            }
        };
    }, [connect]);

    return {
        isConnected,
        isAuthenticated,
        clientId,
        messages,
        sendMessage,
        sendData,
        subscribe,
        publish
    };
};

export default useWebSocket;
```

#### React Dashboard Component
```javascript
// Dashboard.js
import React, { useState, useEffect } from 'react';
import useWebSocket from './hooks/useWebSocket';
import './Dashboard.css';

const Dashboard = () => {
    const {
        isConnected,
        isAuthenticated,
        clientId,
        messages,
        sendData,
        subscribe,
        publish
    } = useWebSocket('ws://localhost:8080', 'your-auth-token');

    const [sensorData, setSensorData] = useState([]);
    const [alerts, setAlerts] = useState([]);

    useEffect(() => {
        if (isAuthenticated) {
            // Subscribe to sensor data and alerts
            subscribe('sensor_updates');
            subscribe('system_alerts');
        }
    }, [isAuthenticated, subscribe]);

    useEffect(() => {
        // Process incoming messages
        const latestMessage = messages[messages.length - 1];
        if (!latestMessage) return;

        if (latestMessage.type === 'publish') {
            if (latestMessage.topic === 'sensor_updates') {
                setSensorData(prev => [...prev.slice(-99), latestMessage.data]);
            } else if (latestMessage.topic === 'system_alerts') {
                setAlerts(prev => [...prev.slice(-19), {
                    ...latestMessage.data,
                    timestamp: new Date().toISOString()
                }]);
            }
        }
    }, [messages]);

    const handleSendTestData = () => {
        sendData({
            component: 'dashboard',
            action: 'test_data',
            value: Math.random() * 100,
            timestamp: new Date().toISOString()
        });
    };

    const handleSendAlert = () => {
        publish('system_alerts', {
            level: 'info',
            message: 'Test alert from dashboard',
            source: 'user_interface'
        });
    };

    return (
        

### Starting the Server
```javascript
const ws = new WebSocketHandler(config, db, window);
await ws.start();
```

### Stopping the Server
```javascript
await ws.stop();
```

### Public API Methods
- `start()` - Start the WebSocket server
- `stop()` - Stop the WebSocket server
- `publish(topic, data, excludeClientId)` - Publish to topic
- `sendToClient(clientId, data)` - Send data to specific client
- `broadcastToAll(message, clientTypeFilter)` - Broadcast to all clients
- `registerRPCMethod(methodName, handler)` - Register RPC method
- `getStatus()` - Get server status and statistics

### Events You Can Listen To
- `server-started` - Server has started
- `server-stopped` - Server has stopped
- `client-connected` - New client connected
- `client-disconnected` - Client disconnected
- `data-received` - Data received from client
- `request` - Request received from client
- `stream-started` - Stream started by client
- `stream-data` - Stream data received
- `database-insert` - Data saved to database

This documentation should help you understand and use the WebSocketHandler library effectively, whether you're building IoT applications, real-time dashboards, chat systems, or any other application requiring real-time communication!