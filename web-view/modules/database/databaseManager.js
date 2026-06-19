// modules/database/databaseManager.js
const FirebaseDB = require('../../App/lib/db/firebaseDB');
const Database = require('../../App/lib/db/mysqlDB');
const { apiKey } = require('../../firebaseConfig');

class DatabaseManager {
    constructor() {
        this.db = null;
        this.useFirebase = process.env.USE_FIREBASE === 'true';
    }

    async initialize() {
        try {
            if (this.useFirebase) {
                this.db = new FirebaseDB({
                    apiKey: process.env.FIREBASE_API_KEY || apiKey,
                    authDomain: process.env.FIREBASE_AUTH_DOMAIN || 'your-auth-domain',
                    databaseURL: process.env.FIREBASE_DATABASE_URL || 'https://your-database-url.firebaseio.com',
                    projectId: process.env.FIREBASE_PROJECT_ID || 'yocmdur-project-id',
                    storageBucket: process.env.FIREBASE_STORAGE_BUCKET || 'your-storage-bucket',
                    messagingSenderId: process.env.FIREBASE_MESSAGING_SENDER_ID || 'your-messaging-sender-id',
                    appId: process.env.FIREBASE_APP_ID || 'your-app-id',
                    measurementId: process.env.FIREBASE_MEASUREMENT_ID || 'your-measurement-id'
                });
            } else {
                this.db = new Database({
                    host: process.env.MYSQL_HOST || 'your-db-host',
                    user: process.env.MYSQL_USER || 'your-db-user',
                    password: process.env.MYSQL_PASSWORD || '',
                    database: process.env.MYSQL_DATABASE || ''
                });
                await this.db.connect();
            }

            console.log(`Database initialized: ${this.useFirebase ? 'Firebase' : 'MySQL'}`);
        } catch (error) {
            console.error('Database initialization failed:', error);
            throw error;
        }
    }

    getDatabase() {
        return this.db;
    }

    async close() {
        if (this.db) {
            try {
                await this.db.close();
                console.log('Database connection closed');
            } catch (error) {
                console.error('Error closing database connection:', error);
                throw error;
            }
        }
    }

    isFirebase() {
        return this.useFirebase;
    }
}

module.exports = DatabaseManager;