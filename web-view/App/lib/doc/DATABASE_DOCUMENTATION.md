# Database Library Documentation

This documentation covers the usage of the Database and QueryBuilder classes implemented in `database.js`.

## Table of Contents
- [Database Class](#database-class)
- [QueryBuilder Class](#querybuilder-class)
- [Examples](#examples)

## Database Class

The Database class provides a wrapper around MySQL connections with additional features like encryption and query building.

### Constructor
```javascript
const db = new Database({
    host: 'localhost',
    user: 'root',
    password: '',
    database: 'your_database'
});
```

### Core Methods

#### `connect()`
Establishes connection to the MySQL database.
```javascript
await db.connect();
```

#### `query(sql, params = [])`
Executes raw SQL queries with parameter binding.
```javascript
const results = await db.query('SELECT * FROM users WHERE id = ?', [1]);
```

#### `table(tableName)` / `from(tableName)`
Creates a new QueryBuilder instance for chainable queries.
```javascript
const users = await db.table('users').where('age', '>', 18).get();
// or
const users = await db.from('users').where('age', '>', 18).get();
```

#### Data Encryption Methods

##### `encrypt(text)`
Encrypts sensitive data.
```javascript
const encrypted = db.encrypt('sensitive data');
```

##### `decrypt(encryptedText)`
Decrypts encrypted data.
```javascript
const decrypted = db.decrypt(encryptedText);
```

### Legacy Methods

#### `postData(tableName, data)`
Inserts new record into specified table.
```javascript
await db.postData('users', {
    name: 'John Doe',
    email: 'john@example.com'
});
```

#### `updateData(tableName, data, whereClause, whereParams)`
Updates existing records.
```javascript
await db.updateData('users', 
    { name: 'John Smith' },
    'id = ?',
    [1]
);
```

#### `getDataByFilters(tableName, filters, options)`
Retrieves data with filters and options.
```javascript
const users = await db.getDataByFilters('users', 
    { role: 'admin' },
    { 
        orderBy: 'created_at DESC',
        limit: 10 
    }
);
```

## QueryBuilder Class

QueryBuilder provides a fluent interface for building SQL queries.

### SELECT Operations

#### Basic Select
```javascript
// Select all users
const users = await db.table('users').get();

// Select specific fields
const names = await db.table('users')
    .select(['name', 'email'])
    .get();
```

### WHERE Clauses

#### Basic Where
```javascript
// Single condition
const activeUsers = await db.table('users')
    .where('status', 'active')
    .get();

// Multiple conditions
const filteredUsers = await db.table('users')
    .where('age', '>', 18)
    .where('status', 'active')
    .get();
```

#### Advanced Where Clauses
```javascript
// WHERE IN
await db.table('users')
    .whereIn('id', [1, 2, 3])
    .get();

// WHERE BETWEEN
await db.table('users')
    .whereBetween('age', 18, 30)
    .get();

// WHERE NULL
await db.table('users')
    .whereNull('deleted_at')
    .get();

// LIKE
await db.table('users')
    .whereLike('name', '%John%')
    .get();
```

### JOIN Operations
```javascript
// Inner Join
const usersWithPosts = await db.table('users')
    .join('posts', 'users.id', '=', 'posts.user_id')
    .get();

// Left Join
const allUsersWithPosts = await db.table('users')
    .leftJoin('posts', 'users.id', '=', 'posts.user_id')
    .get();
```

### ORDER BY and LIMIT
```javascript
// Order By
const sortedUsers = await db.table('users')
    .orderBy('name', 'ASC')
    .get();

// Limit and Skip
const paginatedUsers = await db.table('users')
    .limit(10)
    .skip(20)
    .get();
```

### Aggregates
```javascript
// Count
const userCount = await db.table('users').count();

// First Record
const firstUser = await db.table('users')
    .orderBy('id', 'ASC')
    .first();

// Pluck single column
const emails = await db.table('users')
    .pluck('email');
```

### UPDATE and DELETE
```javascript
// Update
await db.table('users')
    .where('id', 1)
    .update({ status: 'inactive' });

// Delete
await db.table('users')
    .where('status', 'inactive')
    .delete();
```

## Examples

### Complete CRUD Operations Example

```javascript
// Create
await db.table('users').postData({
    name: 'John Doe',
    email: 'john@example.com',
    age: 25
});

// Read with conditions
const adults = await db.table('users')
    .select(['name', 'email'])
    .where('age', '>=', 18)
    .orderBy('name')
    .get();

// Update with conditions
await db.table('users')
    .where('id', 1)
    .update({
        name: 'John Smith',
        updated_at: new Date()
    });

// Delete with conditions
await db.table('users')
    .where('status', 'inactive')
    .delete();
```

### Complex Query Example

```javascript
const results = await db.table('users')
    .select(['users.name', 'orders.total', 'products.title'])
    .join('orders', 'users.id', '=', 'orders.user_id')
    .leftJoin('products', 'orders.product_id', '=', 'products.id')
    .where('users.status', 'active')
    .whereBetween('orders.created_at', '2025-01-01', '2025-12-31')
    .groupBy('users.id')
    .having('orders.total', '>', 1000)
    .orderBy('orders.total', 'DESC')
    .limit(10)
    .get();
```

### Using Encryption for Sensitive Data

```javascript
// Inserting encrypted data
await db.table('users').postData({
    name: db.encrypt('John Doe'),
    email: db.encrypt('john@example.com'),
    password: db.encrypt('secretpassword')
});

// Data is automatically decrypted when retrieved
const user = await db.table('users')
    .where('id', 1)
    .first();
// user.name will be decrypted automatically
```
