// scripts/switch-db.js
const fs = require('fs');
const path = require('path');

const dbType = process.argv[2]; // Get the third argument (e.g., 'mysql' or 'firebase')
const envFilePath = path.join(__dirname, '..', '.env');

if (dbType !== 'mysql' && dbType !== 'firebase') {
    console.error('Invalid database type specified. Use "mysql" or "firebase".');
    process.exit(1);
}

try {
    // Read existing .env file
    let envContent = '';
    if (fs.existsSync(envFilePath)) {
        envContent = fs.readFileSync(envFilePath, 'utf8');
    }

    // Parse existing environment variables
    const envLines = envContent.split('\n');
    const envVars = {};
    const comments = [];
    
    envLines.forEach((line, index) => {
        const trimmedLine = line.trim();
        
        // Skip empty lines and comments, but preserve them
        if (trimmedLine === '' || trimmedLine.startsWith('#')) {
            comments.push({ index, line });
            return;
        }
        
        // Parse key=value pairs
        const equalIndex = line.indexOf('=');
        if (equalIndex !== -1) {
            const key = line.substring(0, equalIndex).trim();
            const value = line.substring(equalIndex + 1);
            envVars[key] = value;
        }
    });

    // Update the USE_FIREBASE value
    if (dbType === 'mysql') {
        envVars['USE_FIREBASE'] = 'false';
        console.log('Switching database configuration to MySQL...');
    } else if (dbType === 'firebase') {
        envVars['USE_FIREBASE'] = 'true';
        console.log('Switching database configuration to Firebase...');
    }

    // Reconstruct the .env file content while preserving structure
    let newEnvContent = '';
    let lineIndex = 0;
    
    // Add comments and sections in original order
    envLines.forEach((originalLine, index) => {
        const trimmedLine = originalLine.trim();
        
        if (trimmedLine === '' || trimmedLine.startsWith('#')) {
            // Preserve comments and empty lines
            newEnvContent += originalLine + '\n';
        } else {
            // Handle environment variables
            const equalIndex = originalLine.indexOf('=');
            if (equalIndex !== -1) {
                const key = originalLine.substring(0, equalIndex).trim();
                if (envVars.hasOwnProperty(key)) {
                    newEnvContent += `${key}=${envVars[key]}\n`;
                    delete envVars[key]; // Remove from remaining vars
                } else {
                    newEnvContent += originalLine + '\n';
                }
            } else {
                newEnvContent += originalLine + '\n';
            }
        }
    });

    // Add any new environment variables that weren't in the original file
    Object.keys(envVars).forEach(key => {
        newEnvContent += `${key}=${envVars[key]}\n`;
    });

    // Write the updated content back to the file
    fs.writeFileSync(envFilePath, newEnvContent.trim() + '\n');
    
    console.log(`Successfully updated .env file to use ${dbType}.`);
    console.log('Environment variables preserved, only USE_FIREBASE setting changed.');
    
    if (dbType === 'firebase') {
        console.log('\n📋 Firebase Development Setup:');
        console.log('To develop with Firebase locally, you may want to:');
        console.log('1. Install Firebase CLI: npm install -g firebase-tools');
        console.log('2. Login to Firebase: firebase login');
        console.log('3. Initialize Firebase project: firebase init');
        console.log('4. Start Firebase emulators: firebase emulators:start');
        console.log('\n📦 Additional Firebase dependencies are already included in package.json');
        console.log('   - firebase: For client SDK');
        console.log('\n💡 For local development, consider adding firebase-admin for server-side operations');
    }
    
    console.log('\n🔄 Please restart your application for the changes to take effect.');
    
} catch (error) {
    console.error('Failed to update .env file:', error);
    process.exit(1);
}