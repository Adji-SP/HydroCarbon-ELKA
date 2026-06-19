// WebSocketHandler.js - Enhanced for General Purpose Applications with Colored Logging
const WebSocket = require('ws');
const crypto = require('crypto');
const EventEmitter = require('events');

class WebSocketHandler extends EventEmitter {
    constructor(config, dbInstance, windowInstance) {
        super();
        
        this.config = {
            port: 8080,
            host: '0.0.0.0',
            enableAuthentication: false,
            authToken: null,
            dbTableName: 'sensors_data',
            requiredFields: [],
            fieldsToEncrypt: [],
            enableHeartbeat: true,
            heartbeatInterval: 30000, // 30 seconds
            maxConnections: 100, // Increased for general apps
            enableDataValidation: true,
            logLevel: 'info',
            
            // New general-purpose features
            enableRequestResponse: true, // Enable request-response pattern
            enablePubSub: true, // Enable publish-subscribe pattern
            enableRPC: true, // Enable remote procedure calls
            enableStreaming: true, // Enable data streaming
            requestTimeout: 10000, // Request timeout in ms
            maxPayloadSize: 10 * 1024 * 1024, // 10MB for larger payloads
            enableCompression: true, // Enable message compression
            enableBinaryData: true, // Support binary data
            clientTypes: ['microcontroller', 'application', 'service'], // Supported client types
            
            ...config
        };

        this.db = dbInstance;
        this.mainWindow = windowInstance;
        this.server = null;
        this.clients = new Map();
        this.isRunning = false;
        this.connectionCount = 0;
        
        // New features for general applications
        this.pendingRequests = new Map(); // For request-response pattern
        this.subscriptions = new Map(); // For pub-sub pattern
        this.rpcMethods = new Map(); // For RPC methods
        this.streams = new Map(); // For streaming data
        
        // Color definitions for console output
        this.colors = {
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
            gray: '\x1b[90m',
            // Background colors
            bgRed: '\x1b[41m',
            bgGreen: '\x1b[42m',
            bgYellow: '\x1b[43m',
            bgBlue: '\x1b[44m',
            bgMagenta: '\x1b[45m',
            bgCyan: '\x1b[46m'
        };

        // Log level colors
        this.logColors = {
            debug: this.colors.gray,
            info: this.colors.cyan,
            warn: this.colors.yellow,
            error: this.colors.red,
            success: this.colors.green,
            highlight: this.colors.magenta
        };
        
        // Generate auth token if authentication is enabled but no token provided
        if (this.config.enableAuthentication && !this.config.authToken) {
            this.config.authToken = this._generateAuthToken();
        }
        
        this._setupRPCMethods();
    }

    // Enhanced logging method with colors
    _log(level, message, data = null) {
        const levels = { debug: 0, info: 1, warn: 2, error: 3, success: 1, highlight: 1 };
        const configLevel = levels[this.config.logLevel] || 1;
        
        if (levels[level] >= configLevel) {
            const timestamp = new Date().toISOString();
            const color = this.logColors[level] || this.colors.white;
            const resetColor = this.colors.reset;
            
            // Create colored prefix based on log level
            let prefix;
            switch(level) {
                case 'debug':
                    prefix = `${this.colors.gray}[DEBUG]${resetColor}`;
                    break;
                case 'info':
                    prefix = `${this.colors.cyan}[INFO]${resetColor}`;
                    break;
                case 'warn':
                    prefix = `${this.colors.yellow}[WARN]${resetColor}`;
                    break;
                case 'error':
                    prefix = `${this.colors.red}[ERROR]${resetColor}`;
                    break;
                case 'success':
                    prefix = `${this.colors.green}[SUCCESS]${resetColor}`;
                    break;
                case 'highlight':
                    prefix = `${this.colors.magenta}[HIGHLIGHT]${resetColor}`;
                    break;
                default:
                    prefix = `${this.colors.white}[${level.toUpperCase()}]${resetColor}`;
            }
            
            // Format timestamp with dim color
            const coloredTimestamp = `${this.colors.dim}${timestamp}${resetColor}`;
            
            // Format the main message with appropriate color
            const coloredMessage = `${color}${message}${resetColor}`;
            
            // Construct the full log message
            const logMessage = `${coloredTimestamp} ${prefix} ${this.colors.bright}[WebSocket]${resetColor} ${coloredMessage}`;
            
            console.log(logMessage);
            
            // If there's additional data, display it with proper formatting
            if (data) {
                if (typeof data === 'object') {
                    console.log(`${this.colors.dim}├─ Data:${resetColor}`);
                    console.log(`${this.colors.gray}${JSON.stringify(data, null, 2)}${resetColor}`);
                } else {
                    console.log(`${this.colors.dim}├─ ${data}${resetColor}`);
                }
            }
        }
    }

    // Specialized logging methods for better UX
    _logConnection(message, clientData = null) {
        const clientInfo = clientData ? `${this.colors.yellow}${clientData.id}${this.colors.reset} (${clientData.ip})` : '';
        this._log('info', `${this.colors.green}🔗 ${message}${this.colors.reset} ${clientInfo}`);
    }

    _logDisconnection(message, clientData = null) {
        const clientInfo = clientData ? `${this.colors.yellow}${clientData.id}${this.colors.reset} (${clientData.ip})` : '';
        this._log('warn', `${this.colors.red}🔌 ${message}${this.colors.reset} ${clientInfo}`);
    }

    _logData(message, clientData = null, dataType = null) {
        const clientInfo = clientData ? `${this.colors.yellow}${clientData.id}${this.colors.reset}` : '';
        const typeInfo = dataType ? `${this.colors.magenta}[${dataType}]${this.colors.reset}` : '';
        this._log('info', `${this.colors.blue}📊 ${message}${this.colors.reset} ${clientInfo} ${typeInfo}`);
    }

    _logAuth(message, success = true, clientData = null) {
        const icon = success ? '🔐' : '❌';
        const color = success ? this.colors.green : this.colors.red;
        const clientInfo = clientData ? `${this.colors.yellow}${clientData.id}${this.colors.reset}` : '';
        this._log(success ? 'success' : 'error', `${color}${icon} ${message}${this.colors.reset} ${clientInfo}`);
    }

    _logServer(message, status = 'info') {
        const icons = {
            started: '🚀',
            stopped: '🛑',
            error: '⚠️',
            info: 'ℹ️'
        };
        const icon = icons[status] || icons.info;
        this._log(status === 'error' ? 'error' : status === 'started' ? 'success' : 'info', 
                 `${icon} ${message}`);
    }

    _logRPC(message, method = null, clientData = null) {
        const methodInfo = method ? `${this.colors.cyan}${method}${this.colors.reset}` : '';
        const clientInfo = clientData ? `${this.colors.yellow}${clientData.id}${this.colors.reset}` : '';
        this._log('info', `${this.colors.magenta}⚡ RPC ${message}${this.colors.reset} ${methodInfo} ${clientInfo}`);
    }

    _logPubSub(message, topic = null, clientData = null) {
        const topicInfo = topic ? `${this.colors.blue}${topic}${this.colors.reset}` : '';
        const clientInfo = clientData ? `${this.colors.yellow}${clientData.id}${this.colors.reset}` : '';
        this._log('info', `${this.colors.cyan}📡 PubSub ${message}${this.colors.reset} ${topicInfo} ${clientInfo}`);
    }

    _logStream(message, streamId = null, clientData = null) {
        const streamInfo = streamId ? `${this.colors.green}${streamId}${this.colors.reset}` : '';
        const clientInfo = clientData ? `${this.colors.yellow}${clientData.id}${this.colors.reset}` : '';
        this._log('info', `${this.colors.green}🌊 Stream ${message}${this.colors.reset} ${streamInfo} ${clientInfo}`);
    }

    // Wait for the ws.Server to finish binding (resolves on 'listening', rejects on 'error')
    _waitForListening(wsServer) {
        return new Promise((resolve, reject) => {
            const onListening = () => {
                wsServer.removeListener('error', onError);
                resolve();
            };
            const onError = (err) => {
                wsServer.removeListener('listening', onListening);
                reject(err);
            };
            wsServer.once('listening', onListening);
            wsServer.once('error', onError);
        });
    }

    // Start WebSocket server
    async start() {
        if (this.isRunning) {
            this._logServer('WebSocket server is already running', 'info');
            return;
        }

        try {
            const serverOptions = {
                port: this.config.port,
                host: this.config.host,
                maxPayload: this.config.maxPayloadSize,
            };

            // Enable compression if configured
            if (this.config.enableCompression) {
                serverOptions.perMessageDeflate = {
                    zlibDeflateOptions: {
                        level: 6,
                        concurrencyLimit: 10,
                    },
                    threshold: 1024,
                };
            }

            this.server = new WebSocket.Server(serverOptions);

            // IMPORTANT: ws.Server binds the port asynchronously.
            // We must await the 'listening' event before declaring success,
            // otherwise EADDRINUSE arrives silently after this function returns.
            await this._waitForListening(this.server);

            // Now the port is actually bound — set up remaining handlers
            this._setupServerEventHandlers();
            this.isRunning = true;
            this.startTime = Date.now();
            
            this._logServer(`Enhanced WebSocket server started on ws://${this.config.host}:${this.config.port}`, 'started');
            this._log('highlight', `Features: ${this.colors.cyan}RPC=${this.config.enableRPC}${this.colors.reset}, ${this.colors.cyan}PubSub=${this.config.enablePubSub}${this.colors.reset}, ${this.colors.cyan}Streaming=${this.config.enableStreaming}${this.colors.reset}`);
            
            if (this.config.enableAuthentication) {
                this._logAuth(`Authentication enabled. Token: ${this.colors.yellow}${this.config.authToken}${this.colors.reset}`, true);
            }

            this._sendToRenderer('websocket-server-status', {
                status: 'started',
                port: this.config.port,
                host: this.config.host,
                authEnabled: this.config.enableAuthentication,
                authToken: this.config.enableAuthentication ? this.config.authToken : null,
                features: {
                    requestResponse: this.config.enableRequestResponse,
                    pubSub: this.config.enablePubSub,
                    rpc: this.config.enableRPC,
                    streaming: this.config.enableStreaming
                },
                timestamp: new Date().toISOString()
            });

            this.emit('server-started', { port: this.config.port, host: this.config.host });

        } catch (error) {
            // Clean up the server object if binding failed
            if (this.server) {
                try { this.server.close(); } catch (_) {}
                this.server = null;
            }
            this.isRunning = false;
            this._logServer(`Failed to start WebSocket server: ${error.message}`, 'error');
            this.emit('server-error', error);
            this._sendToRenderer('websocket-server-error', {
                error: error.message,
                timestamp: new Date().toISOString()
            });
            throw error;
        }
    }

    // Stop WebSocket server
    async stop() {
        if (!this.isRunning) {
            this._logServer('WebSocket server is not running', 'info');
            return;
        }

        try {
            // Clean up pending requests
            this.pendingRequests.forEach((request, requestId) => {
                if (request.timeout) {
                    clearTimeout(request.timeout);
                }
            });
            this.pendingRequests.clear();
            
            // Clean up streams
            this.streams.clear();
            
            // Clean up subscriptions
            this.subscriptions.clear();

            // Close all client connections
            this.clients.forEach((clientData, ws) => {
                if (ws.readyState === WebSocket.OPEN) {
                    ws.close(1000, 'Server shutdown');
                }
            });
            this.clients.clear();

            // Close server
            if (this.server) {
                this.server.close(() => {
                    this._logServer('WebSocket server stopped', 'stopped');
                });
            }

            this.isRunning = false;
            this.connectionCount = 0;

            this._sendToRenderer('websocket-server-status', {
                status: 'stopped',
                timestamp: new Date().toISOString()
            });

            this.emit('server-stopped');

        } catch (error) {
            this._logServer(`Error stopping WebSocket server: ${error.message}`, 'error');
            throw error;
        }
    }

    // Setup server event handlers
    _setupServerEventHandlers() {
        this.server.on('connection', (ws, request) => {
            this._handleNewConnection(ws, request);
        });

        this.server.on('error', (error) => {
            this._logServer(`WebSocket server error: ${error.message}`, 'error');
            this.emit('server-error', error);
            this._sendToRenderer('websocket-server-error', {
                error: error.message,
                timestamp: new Date().toISOString()
            });
        });

        this.server.on('listening', () => {
            this._log('success', `WebSocket server listening on port ${this.colors.yellow}${this.config.port}${this.colors.reset}`);
        });
    }

    // Handle new client connection
    _handleNewConnection(ws, request) {
        const clientIP = request.socket.remoteAddress;
        const userAgent = request.headers['user-agent'] || 'Unknown';
        const clientId = this._generateClientId();

        this._logConnection(`New connection from ${this.colors.blue}${clientIP}${this.colors.reset} (${this.colors.gray}${userAgent}${this.colors.reset})`);

        // Check max connections
        if (this.connectionCount >= this.config.maxConnections) {
            this._log('warn', `${this.colors.red}Max connections (${this.config.maxConnections}) reached, rejecting connection${this.colors.reset}`);
            ws.close(1013, 'Server overloaded');
            return;
        }

        // Store client data
        const clientData = {
            id: clientId,
            ip: clientIP,
            userAgent: userAgent,
            connectedAt: new Date(),
            lastHeartbeat: new Date(),
            isAuthenticated: !this.config.enableAuthentication,
            dataReceived: 0,
            lastDataTime: null,
            clientType: 'unknown', // Will be set during handshake
            subscriptions: new Set(),
            capabilities: new Set()
        };

        this.clients.set(ws, clientData);
        this.connectionCount++;

        // Setup client event handlers
        this._setupClientEventHandlers(ws, clientData);

        // Start heartbeat if enabled
        if (this.config.enableHeartbeat) {
            this._startClientHeartbeat(ws, clientData);
        }

        // Send welcome message with capabilities
        this._sendToClient(ws, {
            type: 'welcome',
            clientId: clientId,
            authRequired: this.config.enableAuthentication,
            serverCapabilities: {
                requestResponse: this.config.enableRequestResponse,
                pubSub: this.config.enablePubSub,
                rpc: this.config.enableRPC,
                streaming: this.config.enableStreaming,
                binaryData: this.config.enableBinaryData,
                compression: this.config.enableCompression
            },
            supportedClientTypes: this.config.clientTypes,
            timestamp: new Date().toISOString()
        });

        this._sendToRenderer('websocket-client-connected', {
            clientId: clientId,
            ip: clientIP,
            userAgent: userAgent,
            totalConnections: this.connectionCount,
            timestamp: new Date().toISOString()
        });

        this.emit('client-connected', { clientId, ip: clientIP, userAgent });
    }

    // Setup client-specific event handlers
    _setupClientEventHandlers(ws, clientData) {
        ws.on('message', (rawData) => {
            this._handleClientMessage(ws, clientData, rawData);
        });

        ws.on('close', (code, reason) => {
            this._handleClientDisconnection(ws, clientData, code, reason);
        });

        ws.on('error', (error) => {
            this._log('error', `Client ${this.colors.yellow}${clientData.id}${this.colors.reset} error: ${error.message}`);
            this.emit('client-error', { clientId: clientData.id, error });
            this._sendToRenderer('websocket-client-error', {
                clientId: clientData.id,
                error: error.message,
                timestamp: new Date().toISOString()
            });
        });

        ws.on('pong', () => {
            clientData.lastHeartbeat = new Date();
            this._log('debug', `💓 Heartbeat received from ${this.colors.yellow}${clientData.id}${this.colors.reset}`);
        });
    }

    // Handle incoming messages from clients
    _handleClientMessage(ws, clientData, rawData) {
        try {
            let message;
            
            // Handle binary data if enabled
            if (this.config.enableBinaryData && rawData instanceof Buffer) {
                // Try to parse as JSON first, otherwise treat as binary
                try {
                    message = JSON.parse(rawData.toString());
                } catch {
                    this._handleBinaryMessage(ws, clientData, rawData);
                    return;
                }
            } else {
                message = JSON.parse(rawData.toString());
            }

            this._log('debug', `📨 Message from ${this.colors.yellow}${clientData.id}${this.colors.reset}:`, message);

            clientData.lastDataTime = new Date();
            clientData.dataReceived++;

            // Handle different message types
            switch (message.type) {
                case 'handshake':
                    this._handleHandshake(ws, clientData, message);
                    break;
                    
                case 'auth':
                    this._handleAuthentication(ws, clientData, message);
                    break;
                    
                case 'sensor_data': // Legacy microcontroller support
                    this._handleSensorData(ws, clientData, message);
                    break;
                    
                case 'data': // General data message
                    this._handleDataMessage(ws, clientData, message);
                    break;
                    
                case 'request':
                    this._handleRequest(ws, clientData, message);
                    break;
                    
                case 'response':
                    this._handleResponse(ws, clientData, message);
                    break;
                    
                case 'subscribe':
                    this._handleSubscribe(ws, clientData, message);
                    break;
                    
                case 'unsubscribe':
                    this._handleUnsubscribe(ws, clientData, message);
                    break;
                    
                case 'publish':
                    this._handlePublish(ws, clientData, message);
                    break;
                    
                case 'rpc_call':
                    this._handleRPCCall(ws, clientData, message);
                    break;
                    
                case 'stream_start':
                    this._handleStreamStart(ws, clientData, message);
                    break;
                    
                case 'stream_data':
                    this._handleStreamData(ws, clientData, message);
                    break;
                    
                case 'stream_end':
                    this._handleStreamEnd(ws, clientData, message);
                    break;
                    
                case 'heartbeat':
                    this._handleHeartbeat(ws, clientData, message);
                    break;
                    
                case 'ping':
                    this._sendToClient(ws, { type: 'pong', timestamp: new Date().toISOString() });
                    break;
                    
                default:
                    this._log('warn', `❓ Unknown message type from ${this.colors.yellow}${clientData.id}${this.colors.reset}: ${this.colors.red}${message.type}${this.colors.reset}`);
                    this._sendToClient(ws, {
                        type: 'error',
                        message: `Unknown message type: ${message.type}`,
                        timestamp: new Date().toISOString()
                    });
            }

        } catch (error) {
            this._log('error', `Error parsing message from ${this.colors.yellow}${clientData.id}${this.colors.reset}: ${error.message}`);
            this._sendToClient(ws, {
                type: 'error',
                message: 'Invalid message format',
                timestamp: new Date().toISOString()
            });
        }
    }

    // Handle client handshake
    _handleHandshake(ws, clientData, message) {
        const { clientType, capabilities, version } = message;
        
        if (clientType && this.config.clientTypes.includes(clientType)) {
            clientData.clientType = clientType;
            this._log('info', `🤝 Client ${this.colors.yellow}${clientData.id}${this.colors.reset} identified as ${this.colors.green}${clientType}${this.colors.reset}`);
        }
        
        if (capabilities && Array.isArray(capabilities)) {
            capabilities.forEach(cap => clientData.capabilities.add(cap));
        }
        
        this._sendToClient(ws, {
            type: 'handshake_response',
            success: true,
            clientId: clientData.id,
            serverVersion: '2.0.0',
            timestamp: new Date().toISOString()
        });

        this.emit('client-handshake', { 
            clientId: clientData.id, 
            clientType, 
            capabilities 
        });
    }

    // Handle authentication (existing method)
    _handleAuthentication(ws, clientData, message) {
        if (!this.config.enableAuthentication) {
            this._sendToClient(ws, {
                type: 'auth_response',
                success: true,
                message: 'Authentication not required',
                timestamp: new Date().toISOString()
            });
            return;
        }

        if (message.token === this.config.authToken) {
            clientData.isAuthenticated = true;
            this._logAuth('Authentication successful', true, clientData);
            
            this._sendToClient(ws, {
                type: 'auth_response',
                success: true,
                message: 'Authentication successful',
                timestamp: new Date().toISOString()
            });

            this._sendToRenderer('websocket-client-authenticated', {
                clientId: clientData.id,
                ip: clientData.ip,
                timestamp: new Date().toISOString()
            });

            this.emit('client-authenticated', { clientId: clientData.id });
        } else {
            this._logAuth('Authentication failed - invalid token', false, clientData);
            
            this._sendToClient(ws, {
                type: 'auth_response',
                success: false,
                message: 'Invalid authentication token',
                timestamp: new Date().toISOString()
            });
            
            setTimeout(() => {
                if (ws.readyState === WebSocket.OPEN) {
                    ws.close(1008, 'Authentication failed');
                }
            }, 1000);
        }
    }

    // Handle sensor data (legacy support for microcontrollers)
    _handleSensorData(ws, clientData, message) {
        // This maintains backward compatibility
        this._handleDataMessage(ws, clientData, {
            ...message,
            type: 'data',
            dataType: 'sensor'
        });
    }

    // Handle general data messages
    _handleDataMessage(ws, clientData, message) {
        if (this.config.enableAuthentication && !clientData.isAuthenticated) {
            this._sendToClient(ws, {
                type: 'error',
                message: 'Authentication required',
                timestamp: new Date().toISOString()
            });
            return;
        }

        try {
            const data = message.data || message.payload || message;
            const dataType = message.dataType || 'general';
            
            // Validate data if enabled
            if (this.config.enableDataValidation && !this._validateData(data, dataType)) {
                this._sendToClient(ws, {
                    type: 'data_response',
                    success: false,
                    message: 'Data validation failed',
                    requestId: message.requestId,
                    timestamp: new Date().toISOString()
                });
                return;
            }

            // Add metadata
            const dataToSave = {
                ...data,
                client_id: clientData.id,
                client_ip: clientData.ip,
                client_type: clientData.clientType,
                data_type: dataType,
                received_at: new Date().toISOString()
            };

            // Save to database if configured
            if (this.db && this.config.dbTableName) {
                this._saveToDatabase(dataToSave, ws, clientData, message.requestId);
            } else {
                // Send success response even if no database
                this._sendToClient(ws, {
                    type: 'data_response',
                    success: true,
                    requestId: message.requestId,
                    timestamp: new Date().toISOString()
                });
            }

            this._logData(`Data received`, clientData, dataType);

            // Emit event for application handling
            this.emit('data-received', {
                clientId: clientData.id,
                clientType: clientData.clientType,
                dataType: dataType,
                data: data,
                timestamp: new Date().toISOString()
            });

            // Send to renderer for real-time display
            this._sendToRenderer('websocket-data-received', {
                clientId: clientData.id,
                clientType: clientData.clientType,
                dataType: dataType,
                data: data,
                timestamp: new Date().toISOString()
            });

        } catch (error) {
            this._log('error', `Error processing data from ${this.colors.yellow}${clientData.id}${this.colors.reset}: ${error.message}`);
            this._sendToClient(ws, {
                type: 'data_response',
                success: false,
                message: 'Error processing data',
                requestId: message.requestId,
                timestamp: new Date().toISOString()
            });
        }
    }

    // Handle request-response pattern
    _handleRequest(ws, clientData, message) {
        if (!this.config.enableRequestResponse) {
            this._sendToClient(ws, {
                type: 'error',
                message: 'Request-response not enabled',
                timestamp: new Date().toISOString()
            });
            return;
        }

        const { requestId, endpoint, data } = message;
        
        this._log('info', `📤 Request from ${this.colors.yellow}${clientData.id}${this.colors.reset}: ${this.colors.cyan}${endpoint}${this.colors.reset}`);
        
        // Emit request event for application handling
        this.emit('request', {
            clientId: clientData.id,
            requestId,
            endpoint,
            data,
            respond: (responseData, error = null) => {
                this._sendToClient(ws, {
                    type: 'response',
                    requestId,
                    success: !error,
                    data: responseData,
                    error: error?.message || null,
                    timestamp: new Date().toISOString()
                });
            }
        });
    }

    // Handle response to our requests
    _handleResponse(ws, clientData, message) {
        const { requestId, success, data, error } = message;
        
        if (this.pendingRequests.has(requestId)) {
            const request = this.pendingRequests.get(requestId);
            clearTimeout(request.timeout);
            this.pendingRequests.delete(requestId);
            
            if (success) {
                request.resolve(data);
            } else {
                request.reject(new Error(error || 'Request failed'));
            }
        }
    }

    // Handle subscription
    _handleSubscribe(ws, clientData, message) {
        if (!this.config.enablePubSub) {
            this._sendToClient(ws, {
                type: 'error',
                message: 'Pub-Sub not enabled',
                timestamp: new Date().toISOString()
            });
            return;
        }

        const { topic } = message;
        
        if (!this.subscriptions.has(topic)) {
            this.subscriptions.set(topic, new Set());
        }
        
        this.subscriptions.get(topic).add(ws);
        clientData.subscriptions.add(topic);
        
        this._sendToClient(ws, {
            type: 'subscribe_response',
            success: true,
            topic,
            timestamp: new Date().toISOString()
        });

        this._logPubSub('subscribed to', topic, clientData);
        this.emit('client-subscribed', { clientId: clientData.id, topic });
    }

    // Handle unsubscription
    _handleUnsubscribe(ws, clientData, message) {
        const { topic } = message;
        
        if (this.subscriptions.has(topic)) {
            this.subscriptions.get(topic).delete(ws);
            if (this.subscriptions.get(topic).size === 0) {
                this.subscriptions.delete(topic);
            }
        }
        
        clientData.subscriptions.delete(topic);
        
        this._sendToClient(ws, {
            type: 'unsubscribe_response',
            success: true,
            topic,
            timestamp: new Date().toISOString()
        });

        this._logPubSub('unsubscribed from', topic, clientData);
        this.emit('client-unsubscribed', { clientId: clientData.id, topic });
    }

    // Handle publish
    _handlePublish(ws, clientData, message) {
        if (!this.config.enablePubSub) {
            this._sendToClient(ws, {
                type: 'error',
                message: 'Pub-Sub not enabled',
                timestamp: new Date().toISOString()
            });
            return;
        }

        const { topic, data } = message;
        
        this.publish(topic, data, clientData.id);
        
        this._sendToClient(ws, {
            type: 'publish_response',
            success: true,
            topic,
            timestamp: new Date().toISOString()
        });
    }

    // Handle RPC calls
    _handleRPCCall(ws, clientData, message) {
        if (!this.config.enableRPC) {
            this._sendToClient(ws, {
                type: 'error',
                message: 'RPC not enabled',
                timestamp: new Date().toISOString()
            });
            return;
        }

        const { requestId, method, params } = message;
        
        this._logRPC('call', method, clientData);
        
        if (this.rpcMethods.has(method)) {
            const rpcMethod = this.rpcMethods.get(method);
            
            try {
                const result = rpcMethod(params, clientData);
                
                if (result instanceof Promise) {
                    result
                        .then(data => {
                            this._sendToClient(ws, {
                                type: 'rpc_response',
                                requestId,
                                success: true,
                                data,
                                timestamp: new Date().toISOString()
                            });
                        })
                        .catch(error => {
                            this._sendToClient(ws, {
                                type: 'rpc_response',
                                requestId,
                                success: false,
                                error: error.message,
                                timestamp: new Date().toISOString()
                            });
                        });
                } else {
                    this._sendToClient(ws, {
                        type: 'rpc_response',
                        requestId,
                        success: true,
                        data: result,
                        timestamp: new Date().toISOString()
                    });
                }
            } catch (error) {
                this._sendToClient(ws, {
                    type: 'rpc_response',
                    requestId,
                    success: false,
                    error: error.message,
                    timestamp: new Date().toISOString()
                });
            }
        } else {
            this._logRPC(`method not found: ${method}`, null, clientData);
            this._sendToClient(ws, {
                type: 'rpc_response',
                requestId,
                success: false,
                error: `Method ${method} not found`,
                timestamp: new Date().toISOString()
            });
        }
    }

    // Handle stream start
    _handleStreamStart(ws, clientData, message) {
        if (!this.config.enableStreaming) {
            this._sendToClient(ws, {
                type: 'error',
                message: 'Streaming not enabled',
                timestamp: new Date().toISOString()
            });
            return;
        }

        const { streamId, streamType } = message;
        
        this.streams.set(streamId, {
            clientId: clientData.id,
            ws: ws,
            streamType: streamType,
            startedAt: new Date()
        });
        
        this._sendToClient(ws, {
            type: 'stream_start_response',
            success: true,
            streamId,
            timestamp: new Date().toISOString()
        });

        this._logStream('started', streamId, clientData);
        this.emit('stream-started', { clientId: clientData.id, streamId, streamType });
    }

    // Handle stream data
    _handleStreamData(ws, clientData, message) {
        const { streamId, data, sequenceNumber } = message;
        
        this._logStream(`data received from stream`, streamId, clientData);
        this.emit('stream-data', {
            clientId: clientData.id,
            streamId,
            data,
            sequenceNumber,
            timestamp: new Date().toISOString()
        });
    }

    // Handle stream end
    _handleStreamEnd(ws, clientData, message) {
        const { streamId } = message;
        
        if (this.streams.has(streamId)) {
            this.streams.delete(streamId);
            this._logStream('ended', streamId, clientData);
            this.emit('stream-ended', { clientId: clientData.id, streamId });
        }
        
        this._sendToClient(ws, {
            type: 'stream_end_response',
            success: true,
            streamId,
            timestamp: new Date().toISOString()
        });
    }

    // Handle binary messages
    _handleBinaryMessage(ws, clientData, buffer) {
        this._log('info', `📦 Binary data received from ${this.colors.yellow}${clientData.id}${this.colors.reset} (${this.colors.cyan}${buffer.length} bytes${this.colors.reset})`);
        this.emit('binary-data', {
            clientId: clientData.id,
            data: buffer,
            timestamp: new Date().toISOString()
        });
    }

    // Handle heartbeat (existing method)
    _handleHeartbeat(ws, clientData, message) {
        clientData.lastHeartbeat = new Date();
        this._sendToClient(ws, {
            type: 'heartbeat_response',
            timestamp: new Date().toISOString()
        });
        this._log('debug', `💓 Heartbeat from ${this.colors.yellow}${clientData.id}${this.colors.reset}`);
    }

    // Public API methods for applications

    // Send request to client and wait for response
    async sendRequest(clientId, endpoint, data, timeout = this.config.requestTimeout) {
        const client = this._getClientByIdAndSocket(clientId);
        if (!client) {
            throw new Error(`Client ${clientId} not found`);
        }

        const requestId = this._generateRequestId();
        
        return new Promise((resolve, reject) => {
            const timeoutHandle = setTimeout(() => {
                this.pendingRequests.delete(requestId);
                reject(new Error('Request timeout'));
            }, timeout);

            this.pendingRequests.set(requestId, {
                resolve,
                reject,
                timeout: timeoutHandle
            });

            this._sendToClient(client.ws, {
                type: 'request',
                requestId,
                endpoint,
                data,
                timestamp: new Date().toISOString()
            });
        });
    }

    // Publish to topic
    publish(topic, data, excludeClientId = null) {
        if (!this.subscriptions.has(topic)) {
            return 0;
        }

        const subscribers = this.subscriptions.get(topic);
        let publishCount = 0;

        subscribers.forEach(ws => {
            const clientData = this.clients.get(ws);
            if (clientData && clientData.id !== excludeClientId && ws.readyState === WebSocket.OPEN) {
                this._sendToClient(ws, {
                    type: 'publish',
                    topic,
                    data,
                    timestamp: new Date().toISOString()
                });
                publishCount++;
            }
        });

        this._logPubSub(`published to ${this.colors.blue}${topic}${this.colors.reset}: ${this.colors.green}${publishCount}${this.colors.reset} subscribers`);
        return publishCount;
    }

    // Register RPC method
    registerRPCMethod(methodName, handler) {
        this.rpcMethods.set(methodName, handler);
        this._logRPC(`method registered: ${this.colors.cyan}${methodName}${this.colors.reset}`);
    }

    // Unregister RPC method
    unregisterRPCMethod(methodName) {
        this.rpcMethods.delete(methodName);
        this._logRPC(`method unregistered: ${this.colors.cyan}${methodName}${this.colors.reset}`);
    }

    // Send data to specific client
    sendToClient(clientId, data) {
        const client = this._getClientByIdAndSocket(clientId);
        if (client) {
            this._sendToClient(client.ws, {
                type: 'data',
                data,
                timestamp: new Date().toISOString()
            });
            return true;
        }
        return false;
    }

    // Broadcast message to all connected clients (existing method enhanced)
    broadcastToAll(message, clientTypeFilter = null) {
        const messageObj = typeof message === 'object' ? message : { data: message };
        messageObj.type = messageObj.type || 'broadcast';
        messageObj.timestamp = new Date().toISOString();
        
        const messageStr = JSON.stringify(messageObj);
        let sentCount = 0;

        this.clients.forEach((clientData, ws) => {
            if (ws.readyState === WebSocket.OPEN) {
                if (!clientTypeFilter || clientData.clientType === clientTypeFilter) {
                    try {
                        ws.send(messageStr);
                        sentCount++;
                    } catch (error) {
                        this._log('error', `Error broadcasting to client ${this.colors.yellow}${clientData.id}${this.colors.reset}: ${error.message}`);
                    }
                }
            }
        });

        this._log('info', `📢 Broadcast sent to ${this.colors.green}${sentCount}${this.colors.reset} clients${clientTypeFilter ? ` (${this.colors.cyan}${clientTypeFilter}${this.colors.reset} only)` : ''}`);
        return sentCount;
    }

    // Start streaming to client
    startStream(clientId, streamId, streamType) {
        const client = this._getClientByIdAndSocket(clientId);
        if (client) {
            this._sendToClient(client.ws, {
                type: 'stream_start',
                streamId,
                streamType,
                timestamp: new Date().toISOString()
            });
            return true;
        }
        return false;
    }

    // Send stream data
    sendStreamData(streamId, data, sequenceNumber = null) {
        if (this.streams.has(streamId)) {
            const stream = this.streams.get(streamId);
            this._sendToClient(stream.ws, {
                type: 'stream_data',
                streamId,
                data,
                sequenceNumber,
                timestamp: new Date().toISOString()
            });
            return true;
        }
        return false;
    }

    // End stream
    endStream(streamId) {
        if (this.streams.has(streamId)) {
            const stream = this.streams.get(streamId);
            this._sendToClient(stream.ws, {
                type: 'stream_end',
                streamId,
                timestamp: new Date().toISOString()
            });
            this.streams.delete(streamId);
            return true;
        }
        return false;
    }

    // Get client by ID
    _getClientByIdAndSocket(clientId) {
        for (const [ws, clientData] of this.clients) {
            if (clientData.id === clientId) {
                return { ws, clientData };
            }
        }
        return null;
    }

    // Validate data based on type
    _validateData(data, dataType) {
        if (!data || typeof data !== 'object') {
            return false;
        }

        // Type-specific validation
        switch (dataType) {
            case 'sensor':
                // Legacy sensor data validation
                return this._validateSensorData(data);
            
            case 'application':
                // Application data validation
                return this._validateApplicationData(data);
            
            case 'service':
                // Service data validation
                return this._validateServiceData(data);
            
            default:
                // General validation - check required fields
                return this._validateSensorData(data); // Fallback to sensor validation
        }
    }

    // Validate sensor data (existing method)
    _validateSensorData(data) {
        if (!data || typeof data !== 'object') {
            return false;
        }

        // Check required fields
        for (const field of this.config.requiredFields) {
            if (data[field] === undefined || data[field] === null || String(data[field]).trim() === '') {
                this._log('warn', `❌ Missing required field: ${this.colors.red}${field}${this.colors.reset}`);
                return false;
            }
        }

        return true;
    }

    // Validate application data
    _validateApplicationData(data) {
        // Application-specific validation logic
        if (!data.hasOwnProperty('payload') && !data.hasOwnProperty('data')) {
            this._log('warn', `❌ Application data must have ${this.colors.cyan}payload${this.colors.reset} or ${this.colors.cyan}data${this.colors.reset} field`);
            return false;
        }
        return true;
    }

    // Validate service data
    _validateServiceData(data) {
        // Service-specific validation logic
        if (!data.hasOwnProperty('service') || !data.hasOwnProperty('operation')) {
            this._log('warn', `❌ Service data must have ${this.colors.cyan}service${this.colors.reset} and ${this.colors.cyan}operation${this.colors.reset} fields`);
            return false;
        }
        return true;
    }

    // Save data to database (enhanced)
    async _saveToDatabase(data, ws, clientData, requestId = null) {
        try {
            let dataToInsert = { ...data };

            // Handle encryption if configured
            if (this.db.encrypt && this.config.fieldsToEncrypt && this.config.fieldsToEncrypt.length > 0) {
                for (const field of this.config.fieldsToEncrypt) {
                    if (dataToInsert.hasOwnProperty(field) && dataToInsert[field] !== null && dataToInsert[field] !== undefined) {
                        try {
                            dataToInsert[field] = this.db.encrypt(String(dataToInsert[field]));
                            this._log('debug', `🔐 Field '${this.colors.cyan}${field}${this.colors.reset}' encrypted`);
                        } catch (encError) {
                            this._log('error', `Error encrypting field '${this.colors.cyan}${field}${this.colors.reset}': ${encError.message}`);
                        }
                    }
                }
            }

            const result = await this.db.postData(this.config.dbTableName, dataToInsert);
            
            this._log('success', `💾 Data saved to database (${this.colors.cyan}${this.config.dbTableName}${this.colors.reset}): ID ${this.colors.yellow}${result.insertId}${this.colors.reset}`);

            // Send success response to client
            this._sendToClient(ws, {
                type: 'data_response',
                success: true,
                insertId: result.insertId,
                requestId: requestId,
                timestamp: new Date().toISOString()
            });

            // Send to renderer
            this._sendToRenderer('websocket-database-insert', {
                clientId: clientData.id,
                table: this.config.dbTableName,
                insertId: result.insertId,
                data: data,
                timestamp: new Date().toISOString()
            });

            // Emit event
            this.emit('database-insert', {
                clientId: clientData.id,
                insertId: result.insertId,
                data: data
            });

        } catch (error) {
            this._log('error', `💥 Database error for client ${this.colors.yellow}${clientData.id}${this.colors.reset}: ${error.message}`);
            
            this._sendToClient(ws, {
                type: 'data_response',
                success: false,
                message: 'Database error',
                requestId: requestId,
                timestamp: new Date().toISOString()
            });

            this._sendToRenderer('websocket-database-error', {
                clientId: clientData.id,
                error: error.message,
                timestamp: new Date().toISOString()
            });

            // Emit event
            this.emit('database-error', {
                clientId: clientData.id,
                error: error
            });
        }
    }

    // Handle client disconnection (enhanced)
    _handleClientDisconnection(ws, clientData, code, reason) {
        this._logDisconnection(`Client disconnected (${code}: ${reason})`, clientData);
        
        // Clean up subscriptions
        clientData.subscriptions.forEach(topic => {
            if (this.subscriptions.has(topic)) {
                this.subscriptions.get(topic).delete(ws);
                if (this.subscriptions.get(topic).size === 0) {
                    this.subscriptions.delete(topic);
                }
            }
        });

        // Clean up streams
        for (const [streamId, stream] of this.streams) {
            if (stream.clientId === clientData.id) {
                this.streams.delete(streamId);
                this._logStream('ended (client disconnected)', streamId, clientData);
                this.emit('stream-ended', { clientId: clientData.id, streamId });
            }
        }

        this.clients.delete(ws);
        this.connectionCount--;

        const connectedDuration = Date.now() - clientData.connectedAt.getTime();
        this._log('info', `📊 Session stats - Duration: ${this.colors.cyan}${Math.round(connectedDuration / 1000)}s${this.colors.reset}, Data received: ${this.colors.yellow}${clientData.dataReceived}${this.colors.reset}, Total connections: ${this.colors.green}${this.connectionCount}${this.colors.reset}`);

        this._sendToRenderer('websocket-client-disconnected', {
            clientId: clientData.id,
            ip: clientData.ip,
            clientType: clientData.clientType,
            connectedDuration: connectedDuration,
            dataReceived: clientData.dataReceived,
            totalConnections: this.connectionCount,
            timestamp: new Date().toISOString()
        });

        this.emit('client-disconnected', {
            clientId: clientData.id,
            clientType: clientData.clientType,
            connectedDuration: connectedDuration,
            dataReceived: clientData.dataReceived
        });
    }

    // Start heartbeat for client (existing method)
    _startClientHeartbeat(ws, clientData) {
        const heartbeatTimer = setInterval(() => {
            if (ws.readyState === WebSocket.OPEN) {
                const timeSinceLastHeartbeat = Date.now() - clientData.lastHeartbeat.getTime();
                
                if (timeSinceLastHeartbeat > this.config.heartbeatInterval * 2) {
                    this._log('warn', `💔 Client ${this.colors.yellow}${clientData.id}${this.colors.reset} heartbeat timeout, closing connection`);
                    ws.close(1000, 'Heartbeat timeout');
                    clearInterval(heartbeatTimer);
                } else {
                    ws.ping();
                }
            } else {
                clearInterval(heartbeatTimer);
            }
        }, this.config.heartbeatInterval);
    }

    // Send message to specific client (existing method)
    _sendToClient(ws, message) {
        if (ws.readyState === WebSocket.OPEN) {
            try {
                ws.send(JSON.stringify(message));
            } catch (error) {
                this._log('error', `Error sending message to client: ${error.message}`);
            }
        }
    }

    // Setup default RPC methods
    _setupRPCMethods() {
        // Server info
        this.registerRPCMethod('server.info', () => ({
            version: '2.0.0',
            features: {
                requestResponse: this.config.enableRequestResponse,
                pubSub: this.config.enablePubSub,
                rpc: this.config.enableRPC,
                streaming: this.config.enableStreaming
            },
            uptime: this.isRunning ? Date.now() - (this.startTime || Date.now()) : 0,
            connections: this.connectionCount
        }));

        // Get server stats
        this.registerRPCMethod('server.stats', () => ({
            totalConnections: this.connectionCount,
            activeStreams: this.streams.size,
            activeSubscriptions: this.subscriptions.size,
            pendingRequests: this.pendingRequests.size,
            uptime: this.isRunning ? Date.now() - (this.startTime || Date.now()) : 0
        }));

        // List topics
        this.registerRPCMethod('pubsub.topics', () => Array.from(this.subscriptions.keys()));

        // List active streams
        this.registerRPCMethod('streams.list', () => {
            const streamList = [];
            for (const [streamId, stream] of this.streams) {
                streamList.push({
                    streamId,
                    clientId: stream.clientId,
                    streamType: stream.streamType,
                    startedAt: stream.startedAt
                });
            }
            return streamList;
        });

        // Echo method for testing
        this.registerRPCMethod('echo', (params) => params);
    }

    // Get server status (enhanced)
    getStatus() {
        const clientsInfo = Array.from(this.clients.values()).map(client => ({
            id: client.id,
            ip: client.ip,
            clientType: client.clientType,
            connectedAt: client.connectedAt,
            isAuthenticated: client.isAuthenticated,
            dataReceived: client.dataReceived,
            lastDataTime: client.lastDataTime,
            subscriptions: Array.from(client.subscriptions),
            capabilities: Array.from(client.capabilities)
        }));

        const topicsInfo = Array.from(this.subscriptions.entries()).map(([topic, subscribers]) => ({
            topic,
            subscriberCount: subscribers.size
        }));

        const streamsInfo = Array.from(this.streams.entries()).map(([streamId, stream]) => ({
            streamId,
            clientId: stream.clientId,
            streamType: stream.streamType,
            startedAt: stream.startedAt
        }));

        return {
            isRunning: this.isRunning,
            port: this.config.port,
            host: this.config.host,
            connectionCount: this.connectionCount,
            maxConnections: this.config.maxConnections,
            authEnabled: this.config.enableAuthentication,
            authToken: this.config.enableAuthentication ? this.config.authToken : null,
            features: {
                requestResponse: this.config.enableRequestResponse,
                pubSub: this.config.enablePubSub,
                rpc: this.config.enableRPC,
                streaming: this.config.enableStreaming,
                binaryData: this.config.enableBinaryData,
                compression: this.config.enableCompression
            },
            clients: clientsInfo,
            topics: topicsInfo,
            streams: streamsInfo,
            pendingRequests: this.pendingRequests.size,
            rpcMethods: Array.from(this.rpcMethods.keys()),
            uptime: this.isRunning ? Date.now() - (this.startTime || Date.now()) : 0
        };
    }

    // Utility methods (existing enhanced)
    _generateAuthToken() {
        return crypto.randomBytes(32).toString('hex');
    }

    _generateClientId() {
        return `client_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }

    _generateRequestId() {
        return `req_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }

    _sendToRenderer(channel, data) {
        if (this.mainWindow && this.mainWindow.webContents) {
            this.mainWindow.webContents.send(channel, data);
        }
    }
}

module.exports = WebSocketHandler;