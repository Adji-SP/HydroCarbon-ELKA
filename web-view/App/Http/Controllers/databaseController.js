let db;
function initializeController(databaseInstance) {
    db = databaseInstance;
}

async function insertSensorData(req, res) {
    const { user_id, device_id, ph_reading, temperature_reading, moisture_percentage } = req.body;

    try {
        db.validate(req.body, {
            user_id: ['required'],
            device_id: ['required']
        });

        const result = await db.postData('sensor_data', {
            user_id,
            device_id,
            ph_reading: db.encrypt(String(ph_reading)),
            temperature_reading: db.encrypt(String(temperature_reading)),
            moisture_percentage: db.encrypt(String(moisture_percentage))
        });

        res.json({ success: true, id: result.insertId, message: "Data received via API and saved." });
    } catch (err) {
        res.status(400).json({ success: false, error: err.message });
    }
};

module.exports = {
    initializeController,
    insertSensorData
};