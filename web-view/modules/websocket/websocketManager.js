// WebsocketManager.js - Enhanced for General Purpose Applications
const WebSocketHandler = require('../../App/lib/com/webSocketCommunicator');

class WebsocketManager {
    constructor(database, mainWindow) {
        this.database = database;
        this.mainWindow = mainWindow;
        this.websocketHandler = null;
        this.config = this.getWebsocketConfig();

        // Application-specific handlers
        this.requestHandlers = new Map();
        this.eventHandlers = new Map();

        // Manager state
        this.isInitialized = false;
        this.startTime = null;

        console.log('WebsocketManager initialized with config:', this.config);
    }

    getWebsocketConfig() {
        return {
            // Convert string to number for numeric values
            port: parseInt(process.env.WEBSOCKET_PORT || process.env.WS_PORT || '8080'),
            host: process.env.WEBSOCKET_HOST || process.env.WS_HOST || '0.0.0.0',

            // Convert string to boolean
            enableAuthentication: (process.env.WEBSOCKET_ENABLE_AUTH || process.env.WS_AUTH_ENABLED) === 'true',
            authToken: process.env.WEBSOCKET_AUTH_TOKEN || process.env.WS_AUTH_TOKEN || null,
            dbTableName: process.env.WEBSOCKET_DB_TABLE_NAME || process.env.WS_DB_TABLE || 'sensors_data',

            // Parse arrays and numbers correctly
            requiredFields: this._parseEnvArray(process.env.WEBSOCKET_REQUIRED_FIELDS || process.env.WS_REQUIRED_FIELDS),
            fieldsToEncrypt: this._parseEnvArray(process.env.WEBSOCKET_FIELDS_TO_ENCRYPT || process.env.WS_ENCRYPT_FIELDS),

            // Convert to boolean and number
            enableHeartbeat: (process.env.WEBSOCKET_ENABLE_HEARTBEAT || process.env.WS_HEARTBEAT_ENABLED) !== 'false',
            heartbeatInterval: parseInt(process.env.WEBSOCKET_HEARTBEAT_INTERVAL || process.env.WS_HEARTBEAT_INTERVAL || '30000'),
            maxConnections: parseInt(process.env.WEBSOCKET_MAX_CONNECTIONS || process.env.WS_MAX_CONNECTIONS || '100'),
            enableDataValidation: (process.env.WEBSOCKET_ENABLE_VALIDATION || process.env.WS_DATA_VALIDATION) !== 'false',
            logLevel: process.env.WEBSOCKET_LOG_LEVEL || process.env.WS_LOG_LEVEL || 'info',

            // New features with proper type conversion
            enableRequestResponse: (process.env.WS_REQUEST_RESPONSE || 'true') !== 'false',
            enablePubSub: (process.env.WS_PUBSUB || 'true') !== 'false',
            enableRPC: (process.env.WS_RPC || 'true') !== 'false',
            enableStreaming: (process.env.WS_STREAMING || 'true') !== 'false',
            requestTimeout: parseInt(process.env.WS_REQUEST_TIMEOUT || '10000'),
            maxPayloadSize: parseInt(process.env.WS_MAX_PAYLOAD_SIZE || (10 * 1024 * 1024).toString()),
            enableCompression: (process.env.WS_COMPRESSION || 'true') !== 'false',
            enableBinaryData: (process.env.WS_BINARY_DATA || 'true') !== 'false',

            // Fix client types - provide default if empty
            clientTypes: this._parseEnvArray(process.env.WS_CLIENT_TYPES) || ['microcontroller', 'application', 'service']
        };
    }

    async initialize() {
        try {
            if (this.isInitialized) {
                console.log('WebSocket manager already initialized');
                return;
            }

            this.websocketHandler = new WebSocketHandler(
                this.config,
                this.database,
                this.mainWindow
            );

            this._setupEventHandlers();
            this._setupDefaultRequestHandlers();
            this._setupDefaultRPCMethods();

            // Wait for the window to load before starting (if window exists)
            const startDelay = this.mainWindow ? 2000 : 0;
            if (startDelay > 0) {
                await new Promise(resolve => setTimeout(resolve, startDelay));
            }

            // Try to start the server, auto-fallback to next port on EADDRINUSE
            await this._startWithPortFallback();

            this.startTime = new Date();
            this.isInitialized = true;
            console.log('Enhanced WebSocket server manager initialized');
        } catch (error) {
            console.error('WebSocket server manager initialization failed:', error);
            throw error;
        }
    }

    // Attempt to start the WebSocket server, retrying on the next port if EADDRINUSE
    async _startWithPortFallback(maxRetries = 5) {
        let attempt = 0;
        let port = this.config.port;

        while (attempt <= maxRetries) {
            try {
                // Update handler config with current port attempt
                this.websocketHandler.config.port = port;
                await this.websocketHandler.start();
                // Success — persist the actual port used
                this.config.port = port;
                if (port !== parseInt(process.env.WS_PORT || '8080')) {
                    console.warn(`⚠️  WS port ${this.config.port - (port - this.config.port)} was in use. WebSocket server started on fallback port ${port}.`);
                    console.warn(`   Update WS_PORT=${port} in your .env to suppress this warning.`);
                }
                return; // bound successfully
            } catch (error) {
                if (error.code === 'EADDRINUSE') {
                    attempt++;
                    console.warn(`⚠️  Port ${port} is already in use (attempt ${attempt}/${maxRetries}). Trying port ${port + 1}...`);
                    port++;
                } else {
                    // Non-port error — re-throw immediately
                    throw error;
                }
            }
        }

        throw new Error(`Could not bind WebSocket server: all ports from ${this.config.port} to ${port - 1} are in use.`);
    }

    // Setup event handlers for the WebSocket server
    _setupEventHandlers() {
        if (!this.websocketHandler) return;

        // Server events
        this.websocketHandler.on('server-started', (data) => {
            console.log(`WebSocket server started on ${data.host}:${data.port}`);
            this._triggerEvent('server-started', data);
        });

        this.websocketHandler.on('server-stopped', () => {
            console.log('WebSocket server stopped');
            this.isInitialized = false;
            this._triggerEvent('server-stopped');
        });

        this.websocketHandler.on('server-error', (error) => {
            console.error('WebSocket server error:', error);
            this._triggerEvent('server-error', error);
        });

        // Client events
        this.websocketHandler.on('client-connected', (data) => {
            console.log(`Client connected: ${data.clientId} (${data.ip})`);
            this._triggerEvent('client-connected', data);
        });

        this.websocketHandler.on('client-disconnected', (data) => {
            console.log(`Client disconnected: ${data.clientId}`);
            this._triggerEvent('client-disconnected', data);
        });

        this.websocketHandler.on('client-authenticated', (data) => {
            console.log(`Client authenticated: ${data.clientId}`);
            this._triggerEvent('client-authenticated', data);
        });

        this.websocketHandler.on('client-handshake', (data) => {
            console.log(`Client handshake: ${data.clientId} (${data.clientType})`);
            this._triggerEvent('client-handshake', data);
        });

        this.websocketHandler.on('client-error', (data) => {
            console.error(`Client error: ${data.clientId}`, data.error);
            this._triggerEvent('client-error', data);
        });

        // Data events
        this.websocketHandler.on('data-received', (data) => {
            console.log(`Data received from ${data.clientId} (${data.dataType})`);
            this._triggerEvent('data-received', data);
        });

        this.websocketHandler.on('binary-data', (data) => {
            console.log(`Binary data received from ${data.clientId}, size: ${data.data.length} bytes`);
            this._triggerEvent('binary-data', data);
        });

        // Request handling
        this.websocketHandler.on('request', (requestData) => {
            this._handleIncomingRequest(requestData);
        });

        // Pub-Sub events
        this.websocketHandler.on('client-subscribed', (data) => {
            console.log(`Client ${data.clientId} subscribed to ${data.topic}`);
            this._triggerEvent('client-subscribed', data);
        });

        this.websocketHandler.on('client-unsubscribed', (data) => {
            console.log(`Client ${data.clientId} unsubscribed from ${data.topic}`);
            this._triggerEvent('client-unsubscribed', data);
        });

        // Streaming events
        this.websocketHandler.on('stream-started', (data) => {
            console.log(`Stream started: ${data.streamId} by ${data.clientId} (${data.streamType})`);
            this._triggerEvent('stream-started', data);
        });

        this.websocketHandler.on('stream-data', (data) => {
            this._triggerEvent('stream-data', data);
        });

        this.websocketHandler.on('stream-ended', (data) => {
            console.log(`Stream ended: ${data.streamId}`);
            this._triggerEvent('stream-ended', data);
        });

        // Database events
        this.websocketHandler.on('database-insert', (data) => {
            console.log(`Database insert: ID ${data.insertId} from ${data.clientId}`);
            this._triggerEvent('database-insert', data);
        });

        this.websocketHandler.on('database-error', (data) => {
            console.error(`Database error from ${data.clientId}:`, data.error);
            this._triggerEvent('database-error', data);
        });
    }

    // Setup default request handlers
    _setupDefaultRequestHandlers() {
        // Health check endpoint
        this.registerRequestHandler('health', async (data, clientInfo) => {
            return {
                status: 'healthy',
                timestamp: new Date().toISOString(),
                server: 'websocket-server',
                version: '2.0.0',
                uptime: this.getUptime()
            };
        });

        // Server status endpoint
        this.registerRequestHandler('status', async (data, clientInfo) => {
            return this.getStatus();
        });

        // Echo endpoint for testing
        this.registerRequestHandler('echo', async (data, clientInfo) => {
            return {
                echo: data,
                clientId: clientInfo.clientId,
                timestamp: new Date().toISOString()
            };
        });

        // Configuration endpoint
        this.registerRequestHandler('config', async (data, clientInfo) => {
            return {
                features: {
                    requestResponse: this.config.enableRequestResponse,
                    pubSub: this.config.enablePubSub,
                    rpc: this.config.enableRPC,
                    streaming: this.config.enableStreaming,
                    binaryData: this.config.enableBinaryData,
                    compression: this.config.enableCompression
                },
                limits: {
                    maxConnections: this.config.maxConnections,
                    maxPayloadSize: this.config.maxPayloadSize,
                    requestTimeout: this.config.requestTimeout,
                    heartbeatInterval: this.config.heartbeatInterval
                },
                supportedClientTypes: this.config.clientTypes
            };
        });

        // Database query endpoint (if database is available)
        if (this.database) {
            this.registerRequestHandler('query', async (data, clientInfo) => {
                const { table, conditions, limit, offset, orderBy } = data;
                if (!table) {
                    throw new Error('Table name is required');
                }

                try {
                    const queryOptions = {
                        conditions: conditions || {},
                        limit: limit || 100,
                        offset: offset || 0,
                        orderBy: orderBy || null
                    };

                    const result = await this.database.getData(table, queryOptions.conditions, queryOptions.limit);
                    return {
                        table,
                        count: result.length,
                        data: result,
                        timestamp: new Date().toISOString()
                    };
                } catch (error) {
                    throw new Error(`Database query failed: ${error.message}`);
                }
            });

            // Insert data endpoint
            this.registerRequestHandler('insert', async (data, clientInfo) => {
                const { table, payload } = data;
                if (!table || !payload) {
                    throw new Error('Table and payload are required');
                }

                try {
                    const result = await this.database.postData(table, {
                        ...payload,
                        client_id: clientInfo.clientId,
                        inserted_at: new Date().toISOString()
                    });
                    return {
                        table,
                        insertId: result.insertId,
                        timestamp: new Date().toISOString()
                    };
                } catch (error) {
                    throw new Error(`Database insert failed: ${error.message}`);
                }
            });

            // Update data endpoint
            this.registerRequestHandler('update', async (data, clientInfo) => {
                const { table, conditions, payload } = data;
                if (!table || !conditions || !payload) {
                    throw new Error('Table, conditions, and payload are required');
                }

                try {
                    const result = await this.database.updateData(table, conditions, {
                        ...payload,
                        updated_at: new Date().toISOString(),
                        updated_by: clientInfo.clientId
                    });
                    return {
                        table,
                        affectedRows: result.affectedRows,
                        timestamp: new Date().toISOString()
                    };
                } catch (error) {
                    throw new Error(`Database update failed: ${error.message}`);
                }
            });

            // Delete data endpoint
            this.registerRequestHandler('delete', async (data, clientInfo) => {
                const { table, conditions } = data;
                if (!table || !conditions) {
                    throw new Error('Table and conditions are required');
                }

                try {
                    const result = await this.database.deleteData(table, conditions);
                    return {
                        table,
                        deletedRows: result.affectedRows,
                        timestamp: new Date().toISOString()
                    };
                } catch (error) {
                    throw new Error(`Database delete failed: ${error.message}`);
                }
            });
        }

        // List connected clients
        this.registerRequestHandler('clients', async (data, clientInfo) => {
            const status = this.getStatus();
            return {
                totalClients: status.connectionCount,
                clients: status.clients.map(client => ({
                    id: client.id,
                    ip: client.ip,
                    clientType: client.clientType,
                    connectedAt: client.connectedAt,
                    isAuthenticated: client.isAuthenticated,
                    dataReceived: client.dataReceived,
                    subscriptions: client.subscriptions.length,
                    capabilities: client.capabilities
                }))
            };
        });

        // List active subscriptions
        this.registerRequestHandler('subscriptions', async (data, clientInfo) => {
            const status = this.getStatus();
            return {
                topics: status.topics,
                totalTopics: status.topics.length
            };
        });

        // List active streams
        this.registerRequestHandler('streams', async (data, clientInfo) => {
            const status = this.getStatus();
            return {
                streams: status.streams,
                totalStreams: status.streams.length
            };
        });
    }

    // Setup default RPC methods
    _setupDefaultRPCMethods() {
        if (!this.websocketHandler) return;

        // File system operations (example)
        this.websocketHandler.registerRPCMethod('fs.list', async (params) => {
            const fs = require('fs').promises;
            const path = require('path');

            const { directory = '.', pattern } = params || {};

            try {
                const files = await fs.readdir(directory);
                let result = files;

                if (pattern) {
                    const regex = new RegExp(pattern);
                    result = files.filter(file => regex.test(file));
                }

                return {
                    directory,
                    files: result,
                    count: result.length
                };
            } catch (error) {
                throw new Error(`File system error: ${error.message}`);
            }
        });

        // System information
        this.websocketHandler.registerRPCMethod('system.info', () => {
            const os = require('os');
            return {
                platform: os.platform(),
                architecture: os.arch(),
                hostname: os.hostname(),
                uptime: os.uptime(),
                memory: {
                    total: os.totalmem(),
                    free: os.freemem()
                },
                cpus: os.cpus().length,
                loadavg: os.loadavg(),
                networkInterfaces: Object.keys(os.networkInterfaces())
            };
        });

        // Process information
        this.websocketHandler.registerRPCMethod('process.info', () => {
            return {
                pid: process.pid,
                version: process.version,
                platform: process.platform,
                arch: process.arch,
                uptime: process.uptime(),
                memory: process.memoryUsage(),
                env: {
                    NODE_ENV: process.env.NODE_ENV,
                    PWD: process.env.PWD
                }
            };
        });

        // Database information (if available)
        if (this.database) {
            this.websocketHandler.registerRPCMethod('database.info', async () => {
                try {
                    const info = await this.database.getConnectionInfo?.() || {};
                    return {
                        connected: !!this.database,
                        type: info.type || 'unknown',
                        ...info
                    };
                } catch (error) {
                    return {
                        connected: false,
                        error: error.message
                    };
                }
            });
        }

        // Manager-specific methods
        this.websocketHandler.registerRPCMethod('manager.restart', async () => {
            try {
                await this.restart();
                return { success: true, message: 'WebSocket server restarted' };
            } catch (error) {
                throw new Error(`Restart failed: ${error.message}`);
            }
        });

        this.websocketHandler.registerRPCMethod('manager.broadcast', (params) => {
            const { message, clientType } = params || {};
            const count = this.broadcastToAll(message, clientType);
            return { success: true, sentTo: count };
        });
    }

    // Handle incoming requests
    async _handleIncomingRequest(requestData) {
        const { clientId, requestId, endpoint, data, respond } = requestData;

        try {
            if (this.requestHandlers.has(endpoint)) {
                const handler = this.requestHandlers.get(endpoint);
                const clientInfo = this._getClientInfo(clientId);

                console.log(`Handling request: ${endpoint} from ${clientId}`);
                const result = await handler(data, clientInfo);
                respond(result);
            } else {
                const error = new Error(`Endpoint '${endpoint}' not found`);
                console.warn(`Unknown endpoint requested: ${endpoint} from ${clientId}`);
                respond(null, error);
            }
        } catch (error) {
            console.error(`Request handler error for ${endpoint}:`, error);
            respond(null, error);
        }
    }

    // Get client information
    _getClientInfo(clientId) {
        const status = this.getStatus();
        const client = status?.clients?.find(c => c.id === clientId);
        return client || { id: clientId };
    }

    // Trigger custom event handlers
    _triggerEvent(eventName, data) {
        if (this.eventHandlers.has(eventName)) {
            const handlers = this.eventHandlers.get(eventName);
            handlers.forEach(handler => {
                try {
                    handler(data);
                } catch (error) {
                    console.error(`Event handler error for ${eventName}:`, error);
                }
            });
        }
    }

    // Parse environment array values
    _parseEnvArray(envValue) {
        if (!envValue) return [];
        return envValue.split(',').map(item => item.trim()).filter(Boolean);
    }

    // Get uptime in milliseconds
    getUptime() {
        return this.startTime ? Date.now() - this.startTime.getTime() : 0;
    }

    // Public API methods

    // Register request handler
    registerRequestHandler(endpoint, handler) {
        this.requestHandlers.set(endpoint, handler);
        console.log(`Request handler registered: ${endpoint}`);
    }

    // Unregister request handler
    unregisterRequestHandler(endpoint) {
        this.requestHandlers.delete(endpoint);
        console.log(`Request handler unregistered: ${endpoint}`);
    }

    // Register event handler
    registerEventHandler(eventName, handler) {
        if (!this.eventHandlers.has(eventName)) {
            this.eventHandlers.set(eventName, new Set());
        }
        this.eventHandlers.get(eventName).add(handler);
        console.log(`Event handler registered: ${eventName}`);
    }

    // Unregister event handler
    unregisterEventHandler(eventName, handler) {
        if (this.eventHandlers.has(eventName)) {
            this.eventHandlers.get(eventName).delete(handler);
            if (this.eventHandlers.get(eventName).size === 0) {
                this.eventHandlers.delete(eventName);
            }
        }
    }

    // Register RPC method
    registerRPCMethod(methodName, handler) {
        if (this.websocketHandler) {
            this.websocketHandler.registerRPCMethod(methodName, handler);
        } else {
            console.warn('WebSocket handler not initialized, RPC method registration deferred');
        }
    }

    // Unregister RPC method
    unregisterRPCMethod(methodName) {
        if (this.websocketHandler) {
            this.websocketHandler.unregisterRPCMethod(methodName);
        }
    }

    // Send request to client
    async sendRequestToClient(clientId, endpoint, data, timeout) {
        if (!this.websocketHandler) {
            throw new Error('WebSocket handler not initialized');
        }
        return await this.websocketHandler.sendRequest(clientId, endpoint, data, timeout);
    }

    // Publish to topic
    publishToTopic(topic, data, excludeClientId = null) {
        if (this.websocketHandler) {
            return this.websocketHandler.publish(topic, data, excludeClientId);
        }
        console.warn('WebSocket handler not initialized');
        return 0;
    }

    // Send data to specific client
    sendToClient(clientId, data) {
        if (this.websocketHandler) {
            return this.websocketHandler.sendToClient(clientId, data);
        }
        console.warn('WebSocket handler not initialized');
        return false;
    }

    // Broadcast to all clients
    broadcastToAll(message, clientTypeFilter = null) {
        if (this.websocketHandler) {
            return this.websocketHandler.broadcastToAll(message, clientTypeFilter);
        }
        console.warn('WebSocket handler not initialized');
        return 0;
    }

    // Start stream
    startStreamToClient(clientId, streamId, streamType) {
        if (this.websocketHandler) {
            return this.websocketHandler.startStream(clientId, streamId, streamType);
        }
        console.warn('WebSocket handler not initialized');
        return false;
    }

    // Send stream data
    sendStreamData(streamId, data, sequenceNumber = null) {
        if (this.websocketHandler) {
            return this.websocketHandler.sendStreamData(streamId, data, sequenceNumber);
        }
        console.warn('WebSocket handler not initialized');
        return false;
    }

    // End stream
    endStream(streamId) {
        if (this.websocketHandler) {
            return this.websocketHandler.endStream(streamId);
        }
        console.warn('WebSocket handler not initialized');
        return false;
    }

    // Get status
    getStatus() {
        const baseStatus = this.websocketHandler ? this.websocketHandler.getStatus() : {
            isRunning: false,
            connectionCount: 0,
            clients: [],
            topics: [],
            streams: [],
            pendingRequests: 0,
            rpcMethods: []
        };

        return {
            ...baseStatus,
            manager: {
                isInitialized: this.isInitialized,
                startTime: this.startTime,
                uptime: this.getUptime(),
                requestHandlers: Array.from(this.requestHandlers.keys()),
                eventHandlers: Array.from(this.eventHandlers.keys()),
                database: !!this.database,
                mainWindow: !!this.mainWindow
            }
        };
    }

    // Restart the WebSocket server
    async restart() {
        console.log('Restarting WebSocket server...');
        try {
            await this.stop();
            await new Promise(resolve => setTimeout(resolve, 1000)); // Wait 1 second
            await this.initialize();
            console.log('WebSocket server restarted successfully');
        } catch (error) {
            console.error('Error restarting WebSocket server:', error);
            throw error;
        }
    }

    // Stop server
    async stop() {
        if (this.websocketHandler) {
            console.log('Stopping WebSocket server...');
            await this.websocketHandler.stop();
            this.websocketHandler = null;
            this.isInitialized = false;
            this.startTime = null;

            // Clear handlers
            this.requestHandlers.clear();
            this.eventHandlers.clear();

            console.log('WebSocket server stopped');
        }
    }

    // Cleanup method
    async cleanup() {
        try {
            await this.stop();
            console.log('WebSocket manager cleanup completed');
        } catch (error) {
            console.error('Error during cleanup:', error);
        }
    }

    // Check if server is running
    isRunning() {
        return this.isInitialized && this.websocketHandler && this.websocketHandler.isRunning;
    }

    // Get configuration
    getConfiguration() {
        return { ...this.config };
    }

    // Update configuration (requires restart to take effect)
    updateConfiguration(newConfig) {
        this.config = { ...this.config, ...newConfig };
        console.log('Configuration updated (restart required for changes to take effect)');
    }

    // Get detailed analytics
    getAnalytics() {
        const status = this.getStatus();
        const clients = status.clients || [];

        return {
            server: {
                uptime: this.getUptime(),
                isRunning: status.isRunning,
                totalConnections: status.connectionCount
            },
            clients: {
                total: clients.length,
                byType: this._groupBy(clients, 'clientType'),
                authenticated: clients.filter(c => c.isAuthenticated).length,
                dataReceived: clients.reduce((sum, c) => sum + (c.dataReceived || 0), 0)
            },
            features: {
                activeTopics: status.topics?.length || 0,
                activeStreams: status.streams?.length || 0,
                pendingRequests: status.pendingRequests || 0,
                rpcMethods: status.rpcMethods?.length || 0
            },
            manager: {
                requestHandlers: this.requestHandlers.size,
                eventHandlers: this.eventHandlers.size
            }
        };
    }

    // Helper method to group array by property
    _groupBy(array, property) {
        return array.reduce((groups, item) => {
            const key = item[property] || 'unknown';
            groups[key] = (groups[key] || 0) + 1;
            return groups;
        }, {});
    }
}

module.exports = WebsocketManager;