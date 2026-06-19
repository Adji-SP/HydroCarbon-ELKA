// modules/csv/csvLogger.js
// Manages CSV recording of serial data to web-view/csv_log/
const fs   = require('fs');
const path = require('path');

class CsvLogger {
    constructor(outputDir) {
        this.outputDir  = outputDir || path.join(__dirname, '../../csv_log');
        this.fileStream = null;
        this.filename   = null;
        this.filepath   = null;
        this.state      = 'idle';   // 'idle' | 'recording' | 'paused'
        this.lineCount  = 0;
        this.startTime  = null;
        this._ensureDir();
    }

    _ensureDir() {
        if (!fs.existsSync(this.outputDir)) {
            fs.mkdirSync(this.outputDir, { recursive: true });
        }
    }

    _generateFilename(port = 'NDIR') {
        const now  = new Date();
        const pad  = n => String(n).padStart(2, '0');
        const date = `${now.getFullYear()}_${pad(now.getMonth()+1)}_${pad(now.getDate())}`;
        const time = `${pad(now.getHours())}.${pad(now.getMinutes())}.${pad(now.getSeconds())}`;
        const safe = (port || 'NDIR').replace(/[/\\:*?"<>|]/g, '-');
        return `${safe}_${date}.${time}.csv`;
    }

    start(port = 'NDIR') {
        if (this.state === 'recording') {
            return { success: false, error: 'Already recording' };
        }
        if (this.state === 'paused') {
            return this.resume();
        }

        this._ensureDir();
        this.filename   = this._generateFilename(port);
        this.filepath   = path.join(this.outputDir, this.filename);
        this.fileStream = fs.createWriteStream(this.filepath, { flags: 'w', encoding: 'utf8' });
        this.state      = 'recording';
        this.lineCount  = 0;
        this.startTime  = Date.now();

        // Write file header
        this.fileStream.write(`# NDIR Monitor — CSV Recording\n`);
        this.fileStream.write(`# Port   : ${port}\n`);
        this.fileStream.write(`# Started: ${new Date().toISOString()}\n`);
        this.fileStream.write(`# RAW  fields: type,timestamp,phase,ch1_raw,ch2_raw,ch3_raw,ch1_filt,ch2_filt,ch3_filt,ema1,ema2,ema3,flag\n`);
        this.fileStream.write(`# PROC fields: type,timestamp,gas_flag,raw_val1,raw_val2,ratio,ratio_filt,baseline,delta1,delta2,response1,response2,ema_out\n`);

        console.log(`[CsvLogger] Recording started → ${this.filepath}`);
        return { success: true, filename: this.filename, filepath: this.filepath };
    }

    pause() {
        if (this.state !== 'recording') {
            return { success: false, error: 'Not currently recording' };
        }
        this.state = 'paused';
        console.log('[CsvLogger] Recording paused');
        return { success: true };
    }

    resume() {
        if (this.state !== 'paused') {
            return { success: false, error: 'Not paused' };
        }
        this.state = 'recording';
        console.log('[CsvLogger] Recording resumed');
        return { success: true };
    }

    stop() {
        if (this.state === 'idle') {
            return { success: false, error: 'Not recording' };
        }
        const result = {
            success:   true,
            filename:  this.filename,
            filepath:  this.filepath,
            lineCount: this.lineCount,
            duration:  this.startTime ? Date.now() - this.startTime : 0,
        };

        if (this.fileStream) {
            this.fileStream.write(`# Stopped  : ${new Date().toISOString()}\n`);
            this.fileStream.write(`# Total lines: ${this.lineCount}\n`);
            this.fileStream.end();
            this.fileStream = null;
        }

        this.state     = 'idle';
        this.filename  = null;
        this.filepath  = null;
        this.lineCount = 0;
        this.startTime = null;

        console.log(`[CsvLogger] Recording stopped — ${result.lineCount} lines → ${result.filename}`);
        return result;
    }

    // Called by SerialCommunicator for every line received
    writeLine(line) {
        if (this.state !== 'recording') return false;
        if (!this.fileStream || !this.fileStream.writable) return false;
        this.fileStream.write(line + '\n');
        this.lineCount++;
        return true;
    }

    getStatus() {
        return {
            state:     this.state,
            filename:  this.filename,
            filepath:  this.filepath,
            lineCount: this.lineCount,
            duration:  this.startTime ? Date.now() - this.startTime : 0,
        };
    }
}

module.exports = CsvLogger;
