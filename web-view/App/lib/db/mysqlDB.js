// /lib/db/mysqlDB.js
const mysql = require('mysql2');
const crypto = require('crypto');

const ALGORITHM = 'aes-256-cbc';
const SECRET_KEY = crypto.createHash('sha256').update(process.env.DB_ENCRYPTION_KEY || '').digest();
const IV_LENGTH = 16;

class QueryBuilder {
    constructor(database, tableName) {
        this.database = database;
        this.tableName = tableName;
        this.whereConditions = [];
        this.whereParams = [];
        this.orderByClause = null;
        this.limitClause = null;
        this.selectFields = '*';
        this.joinClauses = [];
        this.groupByClause = null;
        this.havingConditions = [];
        this.havingParams = [];
    }

    // SELECT methods
    select(fields = '*') {
        if (Array.isArray(fields)) {
            this.selectFields = fields.map(field => `\`${field}\``).join(', ');
        } else if (typeof fields === 'string') {
            this.selectFields = fields === '*' ? '*' : fields.split(',').map(field => `\`${field.trim()}\``).join(', ');
        }
        return this;
    }

    // WHERE methods
    where(field, operator = '=', value = null) {
        if (typeof field === 'object' && field !== null) {
            // Handle object syntax: where({name: 'John', age: 25})
            for (const [key, val] of Object.entries(field)) {
                this.whereConditions.push(`\`${key}\` = ?`);
                this.whereParams.push(val);
            }
        } else if (arguments.length === 2) {
            // Handle where(field, value) syntax
            this.whereConditions.push(`\`${field}\` = ?`);
            this.whereParams.push(operator);
        } else {
            // Handle where(field, operator, value) syntax
            this.whereConditions.push(`\`${field}\` ${operator} ?`);
            this.whereParams.push(value);
        }
        return this;
    }

    whereIn(field, values) {
        if (Array.isArray(values) && values.length > 0) {
            const placeholders = values.map(() => '?').join(', ');
            this.whereConditions.push(`\`${field}\` IN (${placeholders})`);
            this.whereParams.push(...values);
        }
        return this;
    }

    whereNotIn(field, values) {
        if (Array.isArray(values) && values.length > 0) {
            const placeholders = values.map(() => '?').join(', ');
            this.whereConditions.push(`\`${field}\` NOT IN (${placeholders})`);
            this.whereParams.push(...values);
        }
        return this;
    }

    whereBetween(field, min, max) {
        this.whereConditions.push(`\`${field}\` BETWEEN ? AND ?`);
        this.whereParams.push(min, max);
        return this;
    }

    whereNull(field) {
        this.whereConditions.push(`\`${field}\` IS NULL`);
        return this;
    }

    whereNotNull(field) {
        this.whereConditions.push(`\`${field}\` IS NOT NULL`);
        return this;
    }

    whereLike(field, pattern) {
        this.whereConditions.push(`\`${field}\` LIKE ?`);
        this.whereParams.push(pattern);
        return this;
    }

    orWhere(field, operator = '=', value = null) {
        if (this.whereConditions.length === 0) {
            return this.where(field, operator, value);
        }
        
        if (typeof field === 'object' && field !== null) {
            const orConditions = [];
            for (const [key, val] of Object.entries(field)) {
                orConditions.push(`\`${key}\` = ?`);
                this.whereParams.push(val);
            }
            this.whereConditions[this.whereConditions.length - 1] += ` OR (${orConditions.join(' AND ')})`;
        } else if (arguments.length === 2) {
            this.whereConditions[this.whereConditions.length - 1] += ` OR \`${field}\` = ?`;
            this.whereParams.push(operator);
        } else {
            this.whereConditions[this.whereConditions.length - 1] += ` OR \`${field}\` ${operator} ?`;
            this.whereParams.push(value);
        }
        return this;
    }

    // ORDER BY methods
    orderBy(field, direction = 'ASC') {
        direction = direction.toUpperCase() === 'DESC' ? 'DESC' : 'ASC';
        if (this.orderByClause) {
            this.orderByClause += `, \`${field}\` ${direction}`;
        } else {
            this.orderByClause = `\`${field}\` ${direction}`;
        }
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
        if (offset !== null) {
            this.limitClause = `${offset}, ${count}`;
        } else {
            this.limitClause = count.toString();
        }
        return this;
    }

    take(count) {
        return this.limit(count);
    }

    skip(offset) {
        this.limitClause = this.limitClause ? `${offset}, ${this.limitClause}` : `${offset}, 18446744073709551615`;
        return this;
    }

    // JOIN methods
    join(table, firstField, operator = '=', secondField = null) {
        if (arguments.length === 3) {
            // join('users', 'posts.user_id', 'users.id')
            this.joinClauses.push(`INNER JOIN \`${table}\` ON \`${firstField}\` = \`${operator}\``);
        } else {
            // join('users', 'posts.user_id', '=', 'users.id')
            this.joinClauses.push(`INNER JOIN \`${table}\` ON \`${firstField}\` ${operator} \`${secondField}\``);
        }
        return this;
    }

    leftJoin(table, firstField, operator = '=', secondField = null) {
        if (arguments.length === 3) {
            this.joinClauses.push(`LEFT JOIN \`${table}\` ON \`${firstField}\` = \`${operator}\``);
        } else {
            this.joinClauses.push(`LEFT JOIN \`${table}\` ON \`${firstField}\` ${operator} \`${secondField}\``);
        }
        return this;
    }

    rightJoin(table, firstField, operator = '=', secondField = null) {
        if (arguments.length === 3) {
            this.joinClauses.push(`RIGHT JOIN \`${table}\` ON \`${firstField}\` = \`${operator}\``);
        } else {
            this.joinClauses.push(`RIGHT JOIN \`${table}\` ON \`${firstField}\` ${operator} \`${secondField}\``);
        }
        return this;
    }

    // GROUP BY and HAVING
    groupBy(field) {
        if (Array.isArray(field)) {
            this.groupByClause = field.map(f => `\`${f}\``).join(', ');
        } else {
            this.groupByClause = this.groupByClause ? `${this.groupByClause}, \`${field}\`` : `\`${field}\``;
        }
        return this;
    }

    having(field, operator = '=', value = null) {
        if (arguments.length === 2) {
            this.havingConditions.push(`\`${field}\` = ?`);
            this.havingParams.push(operator);
        } else {
            this.havingConditions.push(`\`${field}\` ${operator} ?`);
            this.havingParams.push(value);
        }
        return this;
    }

    // Execution methods
    async get() {
        const sql = this._buildSelectQuery();
        const params = [...this.whereParams, ...this.havingParams];
        
        if (this.limitClause) {
            const limitParts = this.limitClause.split(', ');
            params.push(...limitParts.map(p => parseInt(p)));
        }

        const rows = await this.database.query(sql, params);
        return rows.map(row => this.database._decryptRow(row));
    }

    async first() {
        this.limit(1);
        const results = await this.get();
        return results.length > 0 ? results[0] : null;
    }

    async count(field = '*') {
        const originalSelect = this.selectFields;
        this.selectFields = field === '*' ? 'COUNT(*) as count' : `COUNT(\`${field}\`) as count`;
        
        const sql = this._buildSelectQuery();
        const params = [...this.whereParams, ...this.havingParams];
        
        const result = await this.database.query(sql, params);
        this.selectFields = originalSelect; // Restore original select
        
        return result[0] ? result[0].count : 0;
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

        const columns = Object.keys(data);
        const values = Object.values(data);
        const setClause = columns.map(col => `\`${col}\` = ?`).join(', ');
        
        let sql = `UPDATE \`${this.tableName}\` SET ${setClause}`;
        
        if (this.whereConditions.length > 0) {
            sql += ` WHERE ${this.whereConditions.join(' AND ')}`;
        }
        
        const params = [...values, ...this.whereParams];
        return await this.database.query(sql, params);
    }

    // DELETE method
    async delete() {
        let sql = `DELETE FROM \`${this.tableName}\``;
        
        if (this.whereConditions.length > 0) {
            sql += ` WHERE ${this.whereConditions.join(' AND ')}`;
        }
        
        return await this.database.query(sql, this.whereParams);
    }

    // Build SELECT query
    _buildSelectQuery() {
        let sql = `SELECT ${this.selectFields} FROM \`${this.tableName}\``;
        
        if (this.joinClauses.length > 0) {
            sql += ` ${this.joinClauses.join(' ')}`;
        }
        
        if (this.whereConditions.length > 0) {
            sql += ` WHERE ${this.whereConditions.join(' AND ')}`;
        }
        
        if (this.groupByClause) {
            sql += ` GROUP BY ${this.groupByClause}`;
        }
        
        if (this.havingConditions.length > 0) {
            sql += ` HAVING ${this.havingConditions.join(' AND ')}`;
        }
        
        if (this.orderByClause) {
            sql += ` ORDER BY ${this.orderByClause}`;
        }
        
        if (this.limitClause) {
            sql += ` LIMIT ${this.limitClause}`;
        }
        
        return sql;
    }

    // Raw query method for complex queries
    raw(sql, params = []) {
        return this.database.query(sql, params);
    }
}

class Database {
    constructor(config) {
        this.connection = mysql.createConnection(config);
        this.config = config;
    }

    connect() {
        return new Promise((resolve, reject) => {
            this.connection.connect(err => {
                if (err) {
                    console.error(`MySQL connection failed to ${this.config.host}/${this.config.database}:`, err.message);
                    return reject(err);
                }
                console.log(`Connected to MySQL: ${this.config.user}@${this.config.host}/${this.config.database}`);
                resolve();
            });
        });
    }

    query(sql, params = []) {
        return new Promise((resolve, reject) => {
            this.connection.query(sql, params, (err, results) => {
                if (err) {
                    console.error("Database query error:", err.sqlMessage || err.message);
                    console.error("SQL:", sql);
                    console.error("Params:", params);
                    return reject(err);
                }
                resolve(results);
            });
        });
    }

    // Chainable query builder - NEW
    table(tableName) {
        return new QueryBuilder(this, tableName);
    }

    // Convenience methods for common operations
    from(tableName) {
        return this.table(tableName);
    }

    // Raw query method
    raw(sql, params = []) {
        return this.query(sql, params);
    }

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

    // EXISTING METHODS (unchanged for backward compatibility)
    postData(tableName, data = {}) {
        const dataToInsert = { ...data };
        const columns = Object.keys(dataToInsert);
        const values = Object.values(dataToInsert);
        const placeholders = columns.map(() => '?').join(', ');
        const sql = `INSERT INTO \`${tableName}\` (\`${columns.join('`, `')}\`) VALUES (${placeholders})`;
        return this.query(sql, values);
    }

    updateData(tableName, data = {}, whereClause = '', whereParams = []) {
        const dataToUpdate = { ...data };
        const columns = Object.keys(dataToUpdate);
        const values = Object.values(dataToUpdate);
        const setClause = columns.map(col => `\`${col}\` = ?`).join(', ');
        const sql = `UPDATE \`${tableName}\` SET ${setClause} WHERE ${whereClause}`;
        return this.query(sql, [...values, ...whereParams]);
    }

    _decryptRow(row) {
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

    async getDataByFilters(tableName, filters = {}, options = {}) {
        const keys = Object.keys(filters);
        let sql = `SELECT * FROM \`${tableName}\``;
        const values = [];

        if (keys.length > 0) {
            const conditions = keys.map(key => {
                values.push(filters[key]);
                return `\`${key}\` = ?`;
            }).join(' AND ');
            sql += ` WHERE ${conditions}`;
        }

        if (options.orderBy) {
            if (typeof options.orderBy === 'string') {
                const parts = options.orderBy.trim().split(/\s+/);
                const column = parts[0];
                const direction = parts[1] ? parts[1].toUpperCase() : 'DESC';
                sql += ` ORDER BY \`${column}\` ${direction === 'DESC' ? 'DESC' : 'ASC'}`;
            } else if (options.orderBy.column) {
                const direction = options.orderBy.direction?.toUpperCase() === 'ASC' ? 'ASC' : 'DESC';
                sql += ` ORDER BY \`${options.orderBy.column}\` ${direction}`;
            }
        }

        if (options.limit && Number.isInteger(options.limit) && options.limit > 0) {
            sql += ` LIMIT ?`;
            values.push(options.limit);
        }

        const rows = await this.query(sql, values);
        return rows.map(row => this._decryptRow(row));
    }

    async getAllUsers() {
        const rows = await this.query('SELECT * FROM users');
        return rows.map(row => this._decryptRow(row));
    }

    async insertUser(name, email) {
        const encryptedName = this.encrypt(name);
        const encryptedEmail = this.encrypt(email);
        return this.query('INSERT INTO users (name, email) VALUES (?, ?)', [encryptedName, encryptedEmail]);
    }

    close() {
        return new Promise((resolve, reject) => {
            if (this.connection) {
                this.connection.end(err => {
                    if (err) {
                        console.error('Error closing MySQL connection:', err.message);
                        return reject(err);
                    }
                    console.log('MySQL connection closed.');
                    resolve();
                });
            } else {
                resolve();
            }
        });
    }

    check_up(data) {
        if (!data) {
            return res.status(500).json({ success: false, error: "Database not initialized for controller." });
        }
    }
}

module.exports = Database;