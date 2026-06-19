// lib/firebase.js
const { initializeApp } = require("firebase/app");
const { getDatabase, ref, push, set, get, update, remove, query, orderByChild, orderByKey, limitToFirst, limitToLast, equalTo, startAt, endAt } = require("firebase/database");
const crypto = require('crypto');

const ALGORITHM = 'aes-256-cbc';
const SECRET_KEY = crypto.createHash('sha256').update(process.env.DB_ENCRYPTION_KEY || '').digest();
const IV_LENGTH = 16;

class FirebaseQueryBuilder {
    constructor(database, tableName) {
        this.database = database;
        this.tableName = tableName;
        this.filters = {};
        this.orderField = null;
        this.orderDirection = 'asc';
        this.limitCount = null;
        this.selectFields = null;
    }

    // SELECT methods
    select(fields = '*') {
        if (Array.isArray(fields)) {
            this.selectFields = fields;
        } else if (typeof fields === 'string' && fields !== '*') {
            this.selectFields = fields.split(',').map(field => field.trim());
        }
        return this;
    }

    // WHERE methods
    where(field, operator = '=', value = null) {
        if (typeof field === 'object' && field !== null) {
            // Handle object syntax: where({name: 'John', age: 25})
            Object.assign(this.filters, field);
        } else if (arguments.length === 2) {
            // Handle where(field, value) syntax
            this.filters[field] = { operator: '=', value: operator };
        } else {
            // Handle where(field, operator, value) syntax
            this.filters[field] = { operator, value };
        }
        return this;
    }

    whereIn(field, values) {
        if (Array.isArray(values) && values.length > 0) {
            this.filters[field] = { operator: 'in', value: values };
        }
        return this;
    }

    whereNotIn(field, values) {
        if (Array.isArray(values) && values.length > 0) {
            this.filters[field] = { operator: 'not-in', value: values };
        }
        return this;
    }

    whereBetween(field, min, max) {
        this.filters[field] = { operator: 'between', value: [min, max] };
        return this;
    }

    whereNull(field) {
        this.filters[field] = { operator: 'null' };
        return this;
    }

    whereNotNull(field) {
        this.filters[field] = { operator: 'not-null' };
        return this;
    }

    whereLike(field, pattern) {
        this.filters[field] = { operator: 'like', value: pattern };
        return this;
    }

    orWhere(field, operator = '=', value = null) {
        // Firebase doesn't support OR queries directly, but we can simulate some cases
        // For now, we'll store OR conditions separately and handle them in filtering
        if (!this.orConditions) this.orConditions = [];
        
        if (typeof field === 'object' && field !== null) {
            this.orConditions.push(field);
        } else if (arguments.length === 2) {
            this.orConditions.push({ [field]: { operator: '=', value: operator } });
        } else {
            this.orConditions.push({ [field]: { operator, value } });
        }
        return this;
    }

    // ORDER BY methods
    orderBy(field, direction = 'ASC') {
        this.orderField = field;
        this.orderDirection = direction.toLowerCase() === 'desc' ? 'desc' : 'asc';
        return this;
    }

    orderByDesc(field) {
        return this.orderBy(field, 'DESC');
    }

    orderByAsc(field) {
        return this.orderBy(field, 'ASC');
    }

    // LIMIT methods
    limit(count, offset = null) {
        this.limitCount = count;
        this.offsetCount = offset;
        return this;
    }

    take(count) {
        return this.limit(count);
    }

    skip(offset) {
        this.offsetCount = offset;
        return this;
    }

    // Execution methods
    async get() {
        try {
            const dataRef = ref(this.database.db, this.tableName);
            let firebaseQuery = dataRef;

            // Apply ordering if specified
            if (this.orderField) {
                if (this.orderField === 'key' || this.orderField === '$key') {
                    firebaseQuery = query(firebaseQuery, orderByKey());
                } else {
                    firebaseQuery = query(firebaseQuery, orderByChild(this.orderField));
                }
            }

            // Apply limit
            if (this.limitCount) {
                if (this.orderDirection === 'desc') {
                    firebaseQuery = query(firebaseQuery, limitToLast(this.limitCount));
                } else {
                    firebaseQuery = query(firebaseQuery, limitToFirst(this.limitCount));
                }
            }

            const snapshot = await get(firebaseQuery);
            
            if (!snapshot.exists()) {
                return [];
            }

            let data = [];
            snapshot.forEach(childSnapshot => {
                const item = this.database._decryptRow(childSnapshot.val());
                data.push({ id: childSnapshot.key, ...item });
            });

            // Apply client-side filtering for complex conditions
            data = this._applyClientFilters(data);

            // Apply field selection
            if (this.selectFields && Array.isArray(this.selectFields)) {
                data = data.map(item => {
                    const selected = { id: item.id };
                    this.selectFields.forEach(field => {
                        if (item.hasOwnProperty(field)) {
                            selected[field] = item[field];
                        }
                    });
                    return selected;
                });
            }

            // Handle offset manually since Firebase doesn't support it directly
            if (this.offsetCount && this.offsetCount > 0) {
                data = data.slice(this.offsetCount);
            }

            return data;
        } catch (error) {
            console.error("Firebase get error:", error);
            throw error;
        }
    }

    async first() {
        this.limit(1);
        const results = await this.get();
        return results.length > 0 ? results[0] : null;
    }

    async count(field = '*') {
        const results = await this.get();
        return results.length;
    }

    async exists() {
        const count = await this.count();
        return count > 0;
    }

    async pluck(field) {
        this.select(field);
        const results = await this.get();
        return results.map(row => row[field]);
    }

    // UPDATE method
    async update(data) {
        if (Object.keys(data).length === 0) {
            throw new Error('No data provided for update');
        }

        try {
            // Get all matching records first
            const records = await this.get();
            
            if (records.length === 0) {
                return { affectedRows: 0 };
            }

            const updatePromises = records.map(record => {
                const recordRef = ref(this.database.db, `${this.tableName}/${record.id}`);
                const encryptedData = this.database._encryptData(data);
                return update(recordRef, encryptedData);
            });

            await Promise.all(updatePromises);
            return { affectedRows: records.length };
        } catch (error) {
            console.error("Firebase update error:", error);
            throw error;
        }
    }

    // DELETE method
    async delete() {
        try {
            // Get all matching records first
            const records = await this.get();
            
            if (records.length === 0) {
                return { affectedRows: 0 };
            }

            const deletePromises = records.map(record => {
                const recordRef = ref(this.database.db, `${this.tableName}/${record.id}`);
                return remove(recordRef);
            });

            await Promise.all(deletePromises);
            return { affectedRows: records.length };
        } catch (error) {
            console.error("Firebase delete error:", error);
            throw error;
        }
    }

    // Raw query method (limited functionality in Firebase)
    raw(path, params = []) {
        console.warn("Raw queries are not supported in Firebase. Use Firebase-specific methods instead.");
        return Promise.resolve([]);
    }

    // Apply client-side filters
    _applyClientFilters(data) {
        return data.filter(item => {
            // Apply main filters
            for (const [field, condition] of Object.entries(this.filters)) {
                if (!this._matchesCondition(item, field, condition)) {
                    return false;
                }
            }

            // Apply OR conditions if any
            if (this.orConditions && this.orConditions.length > 0) {
                const orMatch = this.orConditions.some(orCondition => {
                    return Object.entries(orCondition).every(([field, condition]) => {
                        return this._matchesCondition(item, field, condition);
                    });
                });
                if (!orMatch) return false;
            }

            return true;
        });
    }

    _matchesCondition(item, field, condition) {
        const fieldValue = item[field];
        
        if (typeof condition !== 'object' || condition === null) {
            return fieldValue === condition;
        }

        const { operator, value } = condition;

        switch (operator) {
            case '=':
            case '==':
                return fieldValue == value;
            case '!=':
            case '<>':
                return fieldValue != value;
            case '>':
                return fieldValue > value;
            case '>=':
                return fieldValue >= value;
            case '<':
                return fieldValue < value;
            case '<=':
                return fieldValue <= value;
            case 'in':
                return Array.isArray(value) && value.includes(fieldValue);
            case 'not-in':
                return Array.isArray(value) && !value.includes(fieldValue);
            case 'between':
                return Array.isArray(value) && fieldValue >= value[0] && fieldValue <= value[1];
            case 'like':
                const pattern = value.replace(/%/g, '.*').replace(/_/g, '.');
                return new RegExp(pattern, 'i').test(String(fieldValue));
            case 'null':
                return fieldValue === null || fieldValue === undefined;
            case 'not-null':
                return fieldValue !== null && fieldValue !== undefined;
            default:
                return fieldValue === value;
        }
    }
}

class FirebaseDB {
    constructor(config) {
        this.firebaseApp = initializeApp(config);
        this.db = getDatabase(this.firebaseApp);
        this.config = config;
    }

    // Connection method (for compatibility)
    async connect() {
        console.log(`Connected to Firebase: ${this.config.projectId}`);
        return Promise.resolve();
    }

    // Chainable query builder
    table(tableName) {
        return new FirebaseQueryBuilder(this, tableName);
    }

    from(tableName) {
        return this.table(tableName);
    }

    // Raw query method (limited in Firebase)
    raw(path, params = []) {
        console.warn("Raw queries are not fully supported in Firebase. Use Firebase-specific methods instead.");
        return Promise.resolve([]);
    }

    // Validation method (same as MySQL version)
    validate(data, rules) {
        for (const [field, rule] of Object.entries(rules)) {
            if (rule.includes('required') && (data[field] === undefined || data[field] === null || data[field] === '')) {
                throw new Error(`${field} is required`);
            }
            if (rule.includes('email') && data[field] && !/^\S+@\S+\.\S+$/.test(data[field])) {
                throw new Error(`${field} must be a valid email`);
            }
        }
    }

    // Encryption methods (same as MySQL version)
    encrypt(text) {
        if (text === null || typeof text === 'undefined') return text;
        const iv = crypto.randomBytes(IV_LENGTH);
        const cipher = crypto.createCipheriv(ALGORITHM, Buffer.from(SECRET_KEY), iv);
        let encrypted = cipher.update(String(text), 'utf8', 'hex');
        encrypted += cipher.final('hex');
        return iv.toString('hex') + ':' + encrypted;
    }

    decrypt(encryptedText) {
        if (typeof encryptedText !== 'string' || !encryptedText.includes(':')) {
            return encryptedText;
        }
        try {
            const textParts = encryptedText.split(':');
            const iv = Buffer.from(textParts.shift(), 'hex');
            const encryptedData = textParts.join(':');
            const decipher = crypto.createDecipheriv(ALGORITHM, Buffer.from(SECRET_KEY), iv);
            let decrypted = decipher.update(encryptedData, 'hex', 'utf8');
            decrypted += decipher.final('utf8');
            return decrypted;
        } catch (error) {
            return encryptedText;
        }
    }

    _encryptData(data) {
        const encryptedData = {};
        for (const [key, value] of Object.entries(data)) {
            // You can specify which fields to encrypt based on your needs
            // For now, let's assume sensitive fields contain 'password', 'email', 'phone', etc.
            const sensitiveFields = ['password', 'email', 'phone', 'address', 'name'];
            if (sensitiveFields.some(field => key.toLowerCase().includes(field))) {
                encryptedData[key] = this.encrypt(value);
            } else {
                encryptedData[key] = value;
            }
        }
        return encryptedData;
    }

    _decryptRow(row) {
        if (!row || typeof row !== 'object') return row;
        
        const decryptedRow = { ...row };
        for (const key in decryptedRow) {
            if (typeof decryptedRow[key] === 'string' && decryptedRow[key].includes(':')) {
                const originalValue = decryptedRow[key];
                decryptedRow[key] = this.decrypt(originalValue);
                if (decryptedRow[key] !== originalValue && !isNaN(Number(decryptedRow[key]))) {
                    decryptedRow[key] = Number(decryptedRow[key]);
                }
            }
        }
        return decryptedRow;
    }

    // Legacy methods for backward compatibility
    async postData(tableName, data = {}) {
        try {
            const dataRef = ref(this.db, tableName);
            const newDataRef = push(dataRef);
            const encryptedData = this._encryptData(data);
            
            // Add timestamp
            encryptedData.created_at = new Date().toISOString();
            
            await set(newDataRef, encryptedData);
            return { insertId: newDataRef.key, affectedRows: 1 };
        } catch (error) {
            console.error("Firebase postData error:", error);
            throw error;
        }
    }

    async updateData(tableName, data = {}, whereClause = '', whereParams = []) {
        try {
            // For Firebase, we need to parse the whereClause and whereParams
            // This is a simplified implementation
            const filters = this._parseWhereClause(whereClause, whereParams);
            
            return await this.table(tableName)
                .where(filters)
                .update(data);
        } catch (error) {
            console.error("Firebase updateData error:", error);
            throw error;
        }
    }

    async deleteData(tableName, whereClause = '', whereParams = []) {
        try {
            const filters = this._parseWhereClause(whereClause, whereParams);
            
            return await this.table(tableName)
                .where(filters)
                .delete();
        } catch (error) {
            console.error("Firebase deleteData error:", error);
            throw error;
        }
    }

    async getDataByFilters(tableName, filters = {}, options = {}) {
        try {
            let query = this.table(tableName);

            // Apply filters
            if (Object.keys(filters).length > 0) {
                query = query.where(filters);
            }

            // Apply ordering
            if (options.orderBy) {
                if (typeof options.orderBy === 'string') {
                    const parts = options.orderBy.trim().split(/\s+/);
                    const column = parts[0];
                    const direction = parts[1] || 'DESC';
                    query = query.orderBy(column, direction);
                } else if (options.orderBy.column) {
                    const direction = options.orderBy.direction || 'DESC';
                    query = query.orderBy(options.orderBy.column, direction);
                }
            }

            // Apply limit
            if (options.limit && Number.isInteger(options.limit) && options.limit > 0) {
                query = query.limit(options.limit);
            }

            const results = await query.get();
            return results;
        } catch (error) {
            console.error("Firebase getDataByFilters error:", error);
            throw error;
        }
    }

    async getAllUsers() {
        try {
            const results = await this.table('users').get();
            return results;
        } catch (error) {
            console.error("Firebase getAllUsers error:", error);
            throw error;
        }
    }

    async insertUser(name, email) {
        try {
            const result = await this.postData('users', { name, email });
            return result;
        } catch (error) {
            console.error("Firebase insertUser error:", error);
            throw error;
        }
    }

    // Helper method to parse MySQL-style WHERE clauses for Firebase
    _parseWhereClause(whereClause, whereParams = []) {
        if (!whereClause) return {};
        
        const filters = {};
        let paramIndex = 0;
        
        // Simple parsing - you might need to enhance this based on your needs
        const conditions = whereClause.split(' AND ');
        
        conditions.forEach(condition => {
            const match = condition.match(/`?(\w+)`?\s*(=|!=|>|>=|<|<=|LIKE)\s*\?/i);
            if (match && paramIndex < whereParams.length) {
                const field = match[1];
                const operator = match[2].toLowerCase();
                const value = whereParams[paramIndex++];
                
                filters[field] = { operator, value };
            }
        });
        
        return filters;
    }

    async close() {
        // Firebase connections are managed automatically
        console.log('Firebase connection closed.');
        return Promise.resolve();
    }

    check_up(data) {
        if (!data) {
            return { success: false, error: "Database not initialized for controller." };
        }
    }

    // Firebase-specific methods
    async query(path, params = []) {
        console.warn("Direct query method is not applicable to Firebase. Use table() methods instead.");
        return [];
    }
}

module.exports = FirebaseDB;