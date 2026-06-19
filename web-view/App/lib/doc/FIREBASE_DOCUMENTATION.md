# Firebase Database Handler Documentation

## Overview
The FirebaseDB class provides a MySQL-compatible interface for Firebase Realtime Database operations, allowing seamless switching between MySQL and Firebase databases.

## Features
- Query builder interface similar to MySQL
- Data encryption support
- Real-time updates
- Automatic data conversion
- Backward compatibility with MySQL queries

## Configuration

```javascript
const firebaseConfig = {
    apiKey: "your-api-key",
    authDomain: "your-app.firebaseapp.com",
    databaseURL: "https://your-app.firebaseio.com",
    projectId: "your-project-id",
    storageBucket: "your-app.appspot.com",
    messagingSenderId: "your-messaging-id",
    appId: "your-app-id"
};

const db = new FirebaseDB(firebaseConfig);
```

## Query Builder Methods

### Basic Queries

#### Select Data
```javascript
// Select all records
const users = await db.table('users').get();

// Select specific fields
const names = await db.table('users')
    .select(['name', 'email'])
    .get();

// Get first record
const user = await db.table('users')
    .where('active', true)
    .first();
```

### Filtering Data

#### Where Clauses
```javascript
// Basic where
const activeUsers = await db.table('users')
    .where('status', 'active')
    .get();

// Multiple conditions
const filteredUsers = await db.table('users')
    .where('age', '>', 18)
    .where('status', 'active')
    .get();

// Where In
const specificUsers = await db.table('users')
    .whereIn('id', ['1', '2', '3'])
    .get();

// Where Between
const ageRange = await db.table('users')
    .whereBetween('age', 18, 30)
    .get();
```

### Ordering and Limiting

```javascript
// Order by
const orderedUsers = await db.table('users')
    .orderBy('name', 'asc')
    .get();

// Limit results
const topUsers = await db.table('users')
    .limit(10)
    .get();

// Skip and take (pagination)
const page = await db.table('users')
    .skip(10)
    .take(5)
    .get();
```

### Data Modification

#### Insert Data
```javascript
// Insert single record
const result = await db.table('users').postData({
    name: 'John Doe',
    email: 'john@example.com'
});

// Insert with encryption
const sensitiveData = await db.table('users').postData({
    name: db.encrypt('John Doe'),
    email: db.encrypt('john@example.com'),
    password: db.encrypt('secret123')
});
```

#### Update Data
```javascript
// Update records
await db.table('users')
    .where('id', '1')
    .update({
        status: 'inactive',
        updated_at: new Date().toISOString()
    });
```

#### Delete Data
```javascript
// Delete records
await db.table('users')
    .where('status', 'inactive')
    .delete();
```

## Data Encryption

### Encrypting Sensitive Data
```javascript
// Configure encryption
const sensitiveFields = ['password', 'email', 'phone', 'address', 'name'];

// Encrypt data
const encrypted = db.encrypt('sensitive data');

// Decrypt data
const decrypted = db.decrypt(encrypted);

// Automatic encryption on insert
await db.table('users').postData({
    name: 'John Doe',     // Will be automatically encrypted
    email: 'john@example.com', // Will be automatically encrypted
    public_data: 'visible'  // Won't be encrypted
});
```

## Error Handling

```javascript
try {
    const result = await db.table('users')
        .where('id', '1')
        .update({ status: 'inactive' });
} catch (error) {
    console.error('Firebase operation failed:', error);
}
```

## Data Validation

```javascript
// Define validation rules
const rules = {
    email: 'required|email',
    name: 'required'
};

// Validate data
try {
    db.validate({
        email: 'john@example.com',
        name: 'John Doe'
    }, rules);
} catch (error) {
    console.error('Validation failed:', error.message);
}
```

## Legacy Methods Support

```javascript
// Old-style insert
await db.postData('users', {
    name: 'John Doe',
    email: 'john@example.com'
});

// Old-style update
await db.updateData('users', 
    { status: 'inactive' },
    'id = ?',
    ['1']
);

// Old-style get with filters
await db.getDataByFilters('users',
    { status: 'active' },
    { orderBy: 'created_at DESC', limit: 10 }
);
```

## Best Practices

1. **Use Query Builder**
```javascript
// Preferred approach
await db.table('users')
    .where('status', 'active')
    .orderBy('created_at', 'desc')
    .get();

// Instead of raw queries
await db.raw('path/to/data');
```

2. **Handle Encryption Properly**
```javascript
// Define sensitive fields
const sensitiveFields = ['password', 'email', 'phone'];

// Use automatic encryption
await db.table('users').postData({
    email: 'john@example.com',  // Automatically encrypted
    public_info: 'visible'      // Not encrypted
});
```

3. **Use Validation**
```javascript
// Always validate input
db.validate(inputData, {
    email: 'required|email',
    name: 'required'
});
```

4. **Implement Error Handling**
```javascript
try {
    const result = await db.table('users').get();
} catch (error) {
    console.error('Firebase error:', error);
    // Implement proper error handling
}
```

## Performance Considerations

1. **Optimize Queries**
```javascript
// Use specific field selection
await db.table('users')
    .select(['name', 'email'])  // Only fetch needed fields
    .get();

// Use appropriate limits
await db.table('users')
    .limit(10)  // Limit data transfer
    .get();
```

2. **Batch Operations**
```javascript
// Use batch updates when possible
const batch = [];
users.forEach(user => {
    batch.push(db.table('users')
        .where('id', user.id)
        .update({ status: 'active' }));
});
await Promise.all(batch);
```

## Troubleshooting

### Common Issues

1. **Connection Issues**
   - Verify Firebase configuration
   - Check Internet connectivity
   - Verify Firebase rules

2. **Data Synchronization**
   - Check real-time updates configuration
   - Verify data structure
   - Check Firebase console for errors

3. **Performance Issues**
   - Review query optimization
   - Check data structure
   - Verify indexing rules
