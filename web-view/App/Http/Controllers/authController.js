const bcrypt = require('bcryptjs');
let db;

/**
 * Initializes the controller with a database instance.
 * This must be called once when the application starts.
 * @param {object} databaseInstance - The connected database instance.
 */
function initializeController(databaseInstance) {
    db = databaseInstance;
}

/**
 * Handles user login requests.
 * @param {object} req - The Express request object.
 * @param {object} res - The Express response object.
 */
async function login(req, res) {
    const { username, password } = req.body;

    db.validate(req.body, {
        username: ['required'],
        password: ['required']
    });
    try {
        const users = await db.getDataByFilters('users', { username });
        const user = users && users.length > 0 ? users[0] : null;

        if (!user) {
            return res.status(401).json({ success: false, error: 'Invalid email or password.' });
        }

        const passwordIsValid = await bcrypt.compare(password, user.password);

        if (passwordIsValid) {
            const { password, ...userWithoutPassword } = user;
            res.status(200).json({ success: true, user: userWithoutPassword });
        } else {
            res.status(401).json({ success: false, error: 'Invalid email or password.' });
        }
    } catch (error) {
        console.error('Login error:', error);
        res.status(500).json({ success: false, error: 'Internal server error during login.' });
    }
}

/**
 * Handles user registration requests.
 * @param {object} req - The Express request object.
 * @param {object} res - The Express response object.
 */
async function register(req, res) {
    const { username, password } = req.body;

    db.validate(req.body, {
        username: ['required'],
        // email: ['required', 'email'],
        password: ['required']
    });
    try {
        const existingUsers = await db.getDataByFilters('users', { username });
        if (existingUsers && existingUsers.length > 0) {
            return res.status(409).json({ success: false, error: 'User with this email already exists.' });
        }

        const salt = await bcrypt.genSalt(10);
        const hashedPassword = await bcrypt.hash(password, salt);

        const newUser = {
            username,
            password: hashedPassword,
            // role: 'user'
        };

        const result = await db.postData('users', newUser);
        res.status(201).json({ success: true, userId: result.insertId });

    } catch (error) {
        console.error('Registration error:', error);
        res.status(500).json({ success: false, error: 'Internal server error during registration.' });
    }
}

module.exports = {
    initializeController,
    login,
    register,
};