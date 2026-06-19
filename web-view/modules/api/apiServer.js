// modules/api/apiServer.js
const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');

// Controllers
const dbController = require('../../App/Http/Controllers/databaseController');
const authController = require('../../App/Http/Controllers/authController');
const mauiController = require('../../App/Http/Controllers/mauiController');

class APIServer {
    constructor(database) {
        this.app = express();
        this.database = database;
        this.server = null;
        this.port = process.env.API_PORT || 3001;
        
        this.setupMiddleware();
        this.setupRoutes();
        this.initializeControllers();
    }

    setupMiddleware() {
        this.app.use(cors());
        this.app.use(bodyParser.json());
    }

    initializeControllers() {
        dbController.initializeController(this.database);
        authController.initializeController(this.database);
        mauiController.initializeController(this.database);
    }

    setupRoutes() {
        // Authentication Routes
        this.app.post('/api/auth/register', authController.register);
        this.app.post('/api/auth/login', authController.login);

        // Data Routes
        this.app.get('/api/sensor-data', (req, res) => {
            res.json({ success: true, data: { fm1FlowRate: 0, fm1Volume: 0, fm2FlowRate: 0, fm2Volume: 0 }});
        });
        this.app.post('/api/sensor-data', dbController.insertSensorData);
        this.app.post('/api/maui-data', mauiController.genericDataHandler);

        // Chart Routes
        this.app.get('/api/charts/:folder/:name', (req, res) => {
            const chartDir = require('path').join(__dirname, '../../../log/Data/chart');
            const file = require('path').join(chartDir, req.params.folder, req.params.name);
            if (!require('fs').existsSync(file)) return res.status(404).json({ error: 'Chart not found' });
            res.sendFile(file);
        });
        this.app.post('/api/charts/generate', (req, res) => {
            const { spawn } = require('child_process');
            const proc = spawn('python', [
                require('path').join(__dirname, '../../../log/plot.py'),
                '--input', req.body.input || 'data2_raw.csv'
            ]);
            proc.on('close', code => res.json({ success: code === 0 }));
        });

        // Health check
        this.app.get('/api/health', (req, res) => {
            res.json({ 
                success: true, 
                message: 'API server is running',
                timestamp: new Date().toISOString()
            });
        });

        /*
        this.app.get('/api/profile', authenticateToken, async (req, res) => {
            try {
                const userProfile = await this.database.findUserByEmail(req.user.email);
                const { password, ...profileData } = userProfile;
                res.json({ success: true, data: profileData });
            } catch (error) {
                res.status(500).json({ success: false, message: 'Internal server error' });
            }
        });
        */
    }

    start() {
        this.server = this.app.listen(this.port, () => {
            console.log(`API server listening at http://localhost:${this.port}`);
        });
    }

    async stop() {
        if (this.server) {
            return new Promise((resolve) => {
                this.server.close(() => {
                    console.log('API server stopped');
                    resolve();
                });
            });
        }
    }

    getApp() {
        return this.app;
    }

    getPort() {
        return this.port;
    }
}

module.exports = APIServer;