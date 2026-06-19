// controller/genericApiController.js

// This variable will hold the database instance for this controller.
let db;

/**
 * Initializes the controller with the database instance.
 * @param {object} databaseInstance - An instance of the Database class from database.js.
 */
function initializeController(databaseInstance) {
    if (!databaseInstance) {
        throw new Error("Database instance is required for controller initialization.");
    }
    db = databaseInstance;
}

// EXAMPLE
// {
//     "tableName": "activity_logs",
//     "records": [
//         { "user_id": 1, "action": "login", "ip_address": "192.168.1.10" },
//         { "user_id": 2, "action": "data_export", "details": "Exported temperature readings." }
//     ]
// }


/**
 * A flexible API endpoint to insert data into any specified table.
 * It expects a JSON body with a 'tableName' and a 'records' array.
 * @param {object} req - The Express request object.
 * @param {object} res - The Express response object.
 */
async function genericDataHandler(req, res) {
    if (!db) {
        return res.status(500).json({ success: false, error: "Generic API controller has not been initialized." });
    }

    // 1. Get the target table and the data records from the request body.
    const { tableName, records } = req.body;

    // 2. Validate the incoming structure.
    if (!tableName || typeof tableName !== 'string') {
        return res.status(400).json({ success: false, error: "A 'tableName' string is required in the request body." });
    }
    if (!Array.isArray(records) || records.length === 0) {
        return res.status(400).json({ success: false, error: "A non-empty 'records' array is required." });
    }

    // --- IMPORTANT SECURITY NOTE ---
    // Because this function is generic, it does not know which fields to encrypt.
    // Unlike the specific 'insertSensorData' controller, it will NOT apply encryption.
    // For sensitive data, encryption must be handled by the client application
    // BEFORE sending it to this flexible API endpoint.

    const insertedIds = [];
    const errors = [];

    // 3. Loop through each record and use the generic postData function from your Database class.
    for (let index = 0; index < records.length; index++) {
        const record = records[index];
        try {
            // The postData function is now driven entirely by the API request.
            const result = await db.postData(tableName, record);
            insertedIds.push(result.insertId);
        } catch (err) {
            errors.push({ index: index, record: record, error: err.message });
        }
    }

    // 4. Send back a consolidated response.
    if (errors.length > 0) {
        return res.status(400).json({
            success: false,
            message: `Processed ${records.length} records. Some failed to insert into '${tableName}'.`,
            processedCount: insertedIds.length,
            failedCount: errors.length,
            errors: errors
        });
    }

    res.status(201).json({
        success: true,
        message: `Successfully inserted ${insertedIds.length} records into '${tableName}'.`,
        insertedIds: insertedIds
    });
}

module.exports = {
    initializeController,
    genericDataHandler
};