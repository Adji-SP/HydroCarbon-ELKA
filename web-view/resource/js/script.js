/**
 * script.js — NDIR Monitor frontend logic
 *
 * Parses RAW and PROC CSV lines from the Mega firmware:
 *   RAW : type,timestamp,phase,ch1_raw,ch2_raw,ch3_raw,
 *               ch1_filt,ch2_filt,ch3_filt,ema1,ema2,ema3,flag   (13 fields)
 *   PROC: type,timestamp,gas_flag,raw_val1,raw_val2,
 *               ratio,ratio_filt,baseline,delta1,delta2,
 *               response1,response2,ema_out                        (13 fields)
 *
 * Data arrives via Electron IPC 'serial-data-received' { raw, timestamp, port }
 * or via a direct WebSocket connection (ws://...) managed here in the renderer.
 */

'use strict';

// ── RAW field indices ─────────────────────────────────────────────────────────
const RAW = {
  TYPE:      0,
  TS:        1,
  PHASE:     2,
  CH1_RAW:   3,
  CH2_RAW:   4,
  CH3_RAW:   5,
  CH1_FILT:  6,
  CH2_FILT:  7,
  CH3_FILT:  8,
  EMA1:      9,
  EMA2:     10,
  EMA3:     11,
  FLAG:     12,
};

// ── PROC field indices ────────────────────────────────────────────────────────
const PROC = {
  TYPE:       0,
  TS:         1,
  GAS_FLAG:   2,
  RAW_VAL1:   3,
  RAW_VAL2:   4,
  RATIO:      5,
  RATIO_FILT: 6,
  BASELINE:   7,
  DELTA1:     8,
  DELTA2:     9,
  RESP1:     10,
  RESP2:     11,
  EMA_OUT:   12,
};

const ADC_MAX = 1023; // ATmega2560 10-bit

// ── State ─────────────────────────────────────────────────────────────────────
const state = {
  serialConnected: false,
  wsConnected: false,
  logPaused: false,
  totalLines: 0,
  ws: null,              // native WebSocket instance
  lineCountBuffer: 0,    // for rate calculation
  rateInterval: null,
};

// ── Rolling chart buffer ──────────────────────────────────────────────────────
const CHART_MAX = 300;
const chartData = {
  ch1: [], ch1Filt: [],
  ch2: [], ch2Filt: [],
  ch3: [], ch3Filt: [],
};

// ── DOM refs ─────────────────────────────────────────────────────────────────
const $ = id => document.getElementById(id);

const dom = {
  // badges
  badgeSerial:    $('badge-serial'),
  badgeSerialTxt: $('badge-serial-txt'),
  badgeWs:        $('badge-ws'),
  badgeWsTxt:     $('badge-ws-txt'),
  // drawer
  btnMenu:        $('btn-menu'),
  drawer:         $('conn-drawer'),
  // serial controls
  selPort:        $('sel-port'),
  inpBaud:        $('inp-baud'),
  btnScan:        $('btn-scan-ports'),
  btnSerialConn:  $('btn-serial-connect'),
  btnSerialDisc:  $('btn-serial-disconnect'),
  // ws controls
  inpWsUrl:       $('inp-ws-url'),
  inpWsToken:     $('inp-ws-token'),
  btnWsConn:      $('btn-ws-connect'),
  btnWsDisc:      $('btn-ws-disconnect'),
  // gauges
  g1Raw:   $('g1-raw'),   g1Arc: $('g1-arc'),   g1Ema: $('g1-ema'),   g1Filt: $('g1-filt'),   g1Unit: $('g1-unit'),
  g2Raw:   $('g2-raw'),   g2Arc: $('g2-arc'),   g2Ema: $('g2-ema'),   g2Filt: $('g2-filt'),   g2Unit: $('g2-unit'),
  g3Raw:   $('g3-raw'),   g3Arc: $('g3-arc'),   g3Ema: $('g3-ema'),   g3Filt: $('g3-filt'),   g3Unit: $('g3-unit'),
  // proc
  procRatio:      $('proc-ratio'),
  procRatioFilt:  $('proc-ratio-filt'),
  procResp1:      $('proc-resp1'),
  procResp2:      $('proc-resp2'),
  procBaseline:   $('proc-baseline'),
  procEma:        $('proc-ema'),
  barRatio:       $('bar-ratio'),
  barRatioFilt:   $('bar-ratio-filt'),
  barResp1:       $('bar-resp1'),
  barResp2:       $('bar-resp2'),
  barEma:         $('bar-ema'),
  gasFlag:        $('gas-flag'),
  // log
  logScroll:      $('log-scroll'),
  btnPauseLog:    $('btn-pause-log'),
  btnClearLog:    $('btn-clear-log'),
  // recording (topbar)
  btnRecStart:  $('btn-rec-start'),
  btnRecPause:  $('btn-rec-pause'),
  btnRecStop:   $('btn-rec-stop'),
  recDot:       $('rec-dot'),
  recStatus:    $('rec-status'),
  recElapsed:   $('rec-elapsed'),
  // recording (log bar)
  btnRecStart2: $('btn-rec-start2'),
  btnRecPause2: $('btn-rec-pause2'),
  btnRecStop2:  $('btn-rec-stop2'),
  recDot2:      $('rec-dot2'),
  recStatus2:   $('rec-status2'),
  recFilename:  $('rec-filename'),
  recLines:     $('rec-lines'),
  // statusbar
  sbPort:   $('sb-port'),
  sbBaud:   $('sb-baud'),
  sbState:  $('sb-state'),
  sbLines:  $('sb-lines'),
  sbRateVal:$('sb-rate-val'),
  // canvas
  canvasCh1: $('canvas-ch1'),
  canvasCh2: $('canvas-ch2'),
  canvasCh3: $('canvas-ch3'),
  // chart controls
  optUnit: document.getElementsByName('unit'),
  chkFilt: $('chk-filt'),
};

const canvases = [
  { el: dom.canvasCh1, raw: chartData.ch1, filt: chartData.ch1Filt, color: '#2196F3' },
  { el: dom.canvasCh2, raw: chartData.ch2, filt: chartData.ch2Filt, color: '#4CAF50' },
  { el: dom.canvasCh3, raw: chartData.ch3, filt: chartData.ch3Filt, color: '#F44336' },
];

dom.optUnit.forEach(r => r.addEventListener('change', drawAllCharts));
if (dom.chkFilt) dom.chkFilt.addEventListener('change', drawAllCharts);

// ── SVG gauge arc helper (half-circle, 125.6 ≈ π×40) ────────────────────────
const ARC_LEN = 125.66;
function setGaugeArc(arcEl, fraction) {
  const f = Math.max(0, Math.min(1, fraction));
  arcEl.setAttribute('stroke-dashoffset', (ARC_LEN * (1 - f)).toFixed(2));
}

// ── Update RAW gauges (unit-aware: ADC counts or mV) ─────────────────────────
const isMv = () => dom.optUnit && dom.optUnit[1] && dom.optUnit[1].checked;
const adcToMv = adc => Math.round((adc / ADC_MAX) * 5000);
const fmtVal  = (adc) => isMv() ? adcToMv(adc) : adc;
const fmtEma  = (f)   => isMv() ? Math.round((f / ADC_MAX) * 5000) : parseFloat(f).toFixed(1);

function syncGaugeUnits() {
  const unit = isMv() ? 'mV' : 'ADC counts';
  if (dom.g1Unit) dom.g1Unit.textContent = unit;
  if (dom.g2Unit) dom.g2Unit.textContent = unit;
  if (dom.g3Unit) dom.g3Unit.textContent = unit;
}

// re-sync unit labels whenever the radio changes
if (dom.optUnit) Array.from(dom.optUnit).forEach(r => r.addEventListener('change', syncGaugeUnits));

function updateGauges(fields) {
  const ch1adc = +fields[RAW.CH1_RAW];
  const ch2adc = +fields[RAW.CH2_RAW];
  const ch3adc = +fields[RAW.CH3_RAW];
  const ema1   = parseFloat(fields[RAW.EMA1]);
  const ema2   = parseFloat(fields[RAW.EMA2]);
  const ema3   = parseFloat(fields[RAW.EMA3]);
  const filt1  = +fields[RAW.CH1_FILT];
  const filt2  = +fields[RAW.CH2_FILT];
  const filt3  = +fields[RAW.CH3_FILT];
  const MAX    = isMv() ? 5000 : ADC_MAX;

  // Display values
  dom.g1Raw.textContent  = fmtVal(ch1adc);
  dom.g2Raw.textContent  = fmtVal(ch2adc);
  dom.g3Raw.textContent  = fmtVal(ch3adc);
  dom.g1Ema.textContent  = fmtEma(ema1);
  dom.g2Ema.textContent  = fmtEma(ema2);
  dom.g3Ema.textContent  = fmtEma(ema3);
  dom.g1Filt.textContent = fmtVal(filt1);
  dom.g2Filt.textContent = fmtVal(filt2);
  dom.g3Filt.textContent = fmtVal(filt3);
  syncGaugeUnits();

  // Arc fractions
  setGaugeArc(dom.g1Arc, fmtVal(ch1adc) / MAX);
  setGaugeArc(dom.g2Arc, fmtVal(ch2adc) / MAX);
  setGaugeArc(dom.g3Arc, fmtVal(ch3adc) / MAX);

  // Push raw ADC to chart buffer (charts do their own unit conversion)
  chartData.ch1.push(ch1adc); chartData.ch1Filt.push(filt1);
  chartData.ch2.push(ch2adc); chartData.ch2Filt.push(filt2);
  chartData.ch3.push(ch3adc); chartData.ch3Filt.push(filt3);
  if (chartData.ch1.length > CHART_MAX) {
    chartData.ch1.shift(); chartData.ch1Filt.shift();
    chartData.ch2.shift(); chartData.ch2Filt.shift();
    chartData.ch3.shift(); chartData.ch3Filt.shift();
  }
  drawAllCharts();
}

// ── PROC value flash helper ────────────────────────────────────────────────────
function flashEl(el) {
  if (!el) return;
  el.classList.remove('val-updated');
  void el.offsetWidth;                  // force reflow so animation restarts
  el.classList.add('val-updated');
}

// ── Update PROC panel ─────────────────────────────────────────────────────────
function updateProc(fields) {
  const ratio      = parseFloat(fields[PROC.RATIO]);
  const ratioFilt  = parseFloat(fields[PROC.RATIO_FILT]);
  const baseline   = parseFloat(fields[PROC.BASELINE]);
  const resp1      = parseFloat(fields[PROC.RESP1]);
  const resp2      = parseFloat(fields[PROC.RESP2]);
  const emaOut     = parseFloat(fields[PROC.EMA_OUT]);
  const gasFlag    = parseInt(fields[PROC.GAS_FLAG]);

  // Update values with flash animation
  dom.procRatio.textContent     = isNaN(ratio)     ? '—' : ratio.toFixed(4);     flashEl(dom.procRatio);
  dom.procRatioFilt.textContent = isNaN(ratioFilt) ? '—' : ratioFilt.toFixed(4); flashEl(dom.procRatioFilt);
  dom.procResp1.textContent     = isNaN(resp1)     ? '—' : (resp1 >= 0 ? '+' : '') + resp1.toFixed(4);  flashEl(dom.procResp1);
  dom.procResp2.textContent     = isNaN(resp2)     ? '—' : (resp2 >= 0 ? '+' : '') + resp2.toFixed(4);  flashEl(dom.procResp2);
  dom.procBaseline.textContent  = isNaN(baseline)  ? '—' : baseline.toFixed(4);  flashEl(dom.procBaseline);
  dom.procEma.textContent       = isNaN(emaOut)    ? '—' : emaOut.toFixed(1);     flashEl(dom.procEma);

  // bars — ratio clamped 0–3, response ±1 mapped 0–100 %, ema 0–ADC_MAX
  if (!isNaN(ratio))     dom.barRatio.style.width     = Math.min(100, (ratio / 3) * 100).toFixed(1) + '%';
  if (!isNaN(ratioFilt)) dom.barRatioFilt.style.width = Math.min(100, (ratioFilt / 3) * 100).toFixed(1) + '%';
  if (!isNaN(resp1))     dom.barResp1.style.width     = Math.min(100, Math.abs(resp1) * 100).toFixed(1) + '%';
  if (!isNaN(resp2))     dom.barResp2.style.width     = Math.min(100, Math.abs(resp2) * 100).toFixed(1) + '%';
  if (!isNaN(emaOut))   dom.barEma.style.width        = ((emaOut / ADC_MAX) * 100).toFixed(1) + '%';

  // gas flag
  const gf = dom.gasFlag;
  gf.className = 'gas-flag';
  if (gasFlag === 0) {
    gf.classList.add('baseline');
    gf.textContent = '⏳ BASELINE';
  } else if (Math.abs(resp1) > 0.01 || Math.abs(resp2) > 0.01) {
    gf.classList.add('active');
    gf.textContent = '⚠ GAS DETECTED';
  } else {
    gf.classList.add('clear');
    gf.textContent = '✓ CLEAR';
  }
}

// ── Canvas chart ──────────────────────────────────────────────────────────────

function resizeCanvas() {
  canvases.forEach(c => {
    if (!c.el) return;
    const rect = c.el.parentElement.getBoundingClientRect();
    c.el.width  = rect.width - 12; // padding
    c.el.height = rect.height - 24; // header space
  });
}

function drawAllCharts() {
  canvases.forEach(c => drawSingleChart(c));
}

function drawSingleChart(cfg) {
  if (!cfg.el) return;
  const W = cfg.el.width;
  const H = cfg.el.height;
  if (W <= 0 || H <= 0) return;

  const ctx = cfg.el.getContext('2d');
  ctx.clearRect(0, 0, W, H);

  // background
  ctx.fillStyle = 'transparent';
  ctx.fillRect(0, 0, W, H);

  // grid lines
  ctx.strokeStyle = '#1a1e30';
  ctx.lineWidth = 0.5;
  for (let i = 0; i <= 4; i++) {
    const y = (H / 4) * i;
    ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(W, y); ctx.stroke();
  }

  const n = cfg.raw.length;
  if (n < 2) return;

  const xStep = W / (CHART_MAX - 1);
  const isMv = dom.optUnit && dom.optUnit[1] && dom.optUnit[1].checked;
  const MAX_VAL = isMv ? 5000 : ADC_MAX;
  const showFilt = dom.chkFilt && dom.chkFilt.checked;

  const drawLine = (dataArr, style, width) => {
    ctx.beginPath();
    ctx.strokeStyle = style;
    ctx.lineWidth = width;
    dataArr.forEach((v, i) => {
      const val = isMv ? (v / ADC_MAX) * 5000 : v;
      const x = (i + (CHART_MAX - n)) * xStep;
      const y = H - (val / MAX_VAL) * H;
      i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
    });
    ctx.stroke();
  };

  if (showFilt) drawLine(cfg.filt, '#e0e4ff', 1.0); // Light color for filter
  drawLine(cfg.raw, cfg.color, 1.5);

  // Y-axis labels
  ctx.fillStyle = '#404060';
  ctx.font = '9px Consolas';
  ctx.textAlign = 'left';
  for (let i = 0; i <= 4; i++) {
    const val = Math.round(MAX_VAL - (MAX_VAL / 4) * i);
    const y   = (H / 4) * i;
    ctx.fillText(val + (isMv ? ' mV' : ''), 4, y + (i === 0 ? 10 : (i === 4 ? -2 : 4)));
  }
}

window.addEventListener('resize', () => { resizeCanvas(); drawAllCharts(); });
setTimeout(() => { resizeCanvas(); drawAllCharts(); }, 100);

// ── Log helper ────────────────────────────────────────────────────────────────
const MAX_LOG_LINES = 300;

function appendLog(raw, ts, cssClass = '') {
  if (state.logPaused) return;

  const line = document.createElement('div');
  line.className = 'log-line';
  line.innerHTML =
    `<span class="log-ts">${ts || new Date().toLocaleTimeString()}</span>` +
    `<span class="log-raw ${cssClass}">${escHtml(raw)}</span>`;

  dom.logScroll.appendChild(line);

  // trim old lines
  while (dom.logScroll.children.length > MAX_LOG_LINES) {
    dom.logScroll.removeChild(dom.logScroll.firstChild);
  }

  dom.logScroll.scrollTop = dom.logScroll.scrollHeight;
}

function appendInfo(msg)  { appendLog(msg, null, 'type-info'); }
function appendError(msg) { appendLog(msg, null, 'type-err'); }
function appendWarn(msg)  { appendLog(msg, null, 'type-warn'); }

function escHtml(s) {
  return String(s)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');
}

// ── CSV line parser ───────────────────────────────────────────────────────────
function parseLine(raw, ts) {
  if (!raw) return;
  const line = raw.trim();
  if (!line) return;

  state.totalLines++;
  state.lineCountBuffer++;
  dom.sbLines.textContent = state.totalLines;

  // Recording line counter
  if (rec.status === 'recording') {
    rec.lineCount++;
    if (dom.recLines) dom.recLines.textContent = rec.lineCount + ' lines';
  }

  const fields = line.split(',');
  const type = fields[0];

  if (type === 'RAW' && fields.length === 13) {
    appendLog(line, ts, 'type-RAW');
    updateGauges(fields);
  } else if (type === 'PROC' && fields.length === 13) {
    appendLog(line, ts, 'type-PROC');
    updateProc(fields);
  } else if ((type === 'RAW' || type === 'PROC') && fields.length !== 13) {
    // Bug 3 fix: malformed RAW/PROC line — right prefix but wrong field count
    appendWarn(`[PARSE] Malformed ${type} line (${fields.length}/13 fields): ${escHtml(line)}`);
  } else {
    // Non-CSV header lines, warnings, etc.
    const cls = line.startsWith('WRN') ? 'type-warn'
              : line.startsWith('ERR') ? 'type-err'
              : 'type-info';
    appendLog(line, ts, cls);
  }
}

// ── Serial badge / status bar helpers ────────────────────────────────────────
function setSerialBadge(cls, label) {
  dom.badgeSerial.className = 'conn-badge ' + cls;
  dom.badgeSerialTxt.textContent = label || 'SERIAL';
}

function setWsBadge(cls, label) {
  dom.badgeWs.className = 'conn-badge ' + cls;
  dom.badgeWsTxt.textContent = label || 'WS';
}

function setStatusBar(portPath, baud, stateStr) {
  dom.sbPort.textContent  = portPath || '—';
  dom.sbBaud.textContent  = baud     || '—';
  dom.sbState.textContent = stateStr || '—';
}

// ── IPC → Serial events (Electron only) ──────────────────────────────────────
function setupElectronIPC() {
  if (!window.api) {
    appendWarn('[IPC] window.api not available — running outside Electron?');
    return;
  }

  // Live data
  window.api.receive('serial-data-received', ({ raw, timestamp, port }) => {
    parseLine(raw, timestamp);
  });

  // Connection state changes
  window.api.receive('serial-port-status', (info) => {
    const s = info.state;
    if (s === 'connected') {
      state.serialConnected = true;
      setSerialBadge('connected', 'SERIAL');
      setStatusBar(info.port || dom.selPort.value, dom.inpBaud.value, 'CONNECTED');
      appendInfo(`[SERIAL] Connected → ${info.message}`);
    } else if (s === 'connecting' || s === 'reconnecting') {
      state.serialConnected = false;
      setSerialBadge('connecting', 'SERIAL');
      setStatusBar(null, null, s.toUpperCase());
      appendWarn(`[SERIAL] ${info.message}`);
    } else {
      state.serialConnected = false;
      setSerialBadge('', 'SERIAL');
      setStatusBar(null, null, s.toUpperCase());
    }
  });

  // Error
  window.api.receive('serial-port-error', (msg) => {
    appendError('[SERIAL ERR] ' + msg);
    setSerialBadge('error', 'SERIAL');
    setStatusBar(null, null, 'ERROR');
  });

  // Lost connection
  window.api.receive('serial-connection-lost', (info) => {
    state.serialConnected = false;
    setSerialBadge('error', 'SERIAL');
    appendError(`[SERIAL] Connection lost: ${info.port}`);
    setStatusBar(null, null, 'LOST');
  });

  // Reconnect status
  window.api.receive('serial-reconnect-status', (info) => {
    if (info.status === 'attempting') {
      appendWarn(`[SERIAL] Reconnect attempt ${info.attempts}/${info.maxAttempts}…`);
    } else if (info.status === 'max_attempts_reached') {
      appendError(`[SERIAL] Max reconnect attempts reached.`);
    }
  });

  // Fetch initial serial status
  window.api.invoke('serial-get-status').then(res => {
    if (res && res.success && res.data) {
      const d = res.data;
      if (d.isConnected) {
        state.serialConnected = true;
        setSerialBadge('connected', 'SERIAL');
        setStatusBar(d.currentPortPath, null, 'CONNECTED');
      }
      appendInfo(`[SERIAL] Status: ${d.state} | port: ${d.currentPortPath || 'none'}`);
    }
  }).catch(() => {});
}

// ── Serial control buttons ────────────────────────────────────────────────────
dom.btnSerialConn.addEventListener('click', async () => {
  if (!window.api) return appendError('No IPC bridge');
  const selectedPort = dom.selPort.value || null;  // null = auto-detect
  const baud = parseInt(dom.inpBaud.value) || 115200;
  appendInfo(`[SERIAL] Initiating connection… port=${selectedPort || 'auto'} baud=${baud}`);
  setSerialBadge('connecting', 'SERIAL');
  setStatusBar(selectedPort || 'auto', baud, 'CONNECTING');
  const res = await window.api.invoke('serial-force-reconnect');
  if (!res.success) {
    appendError('[SERIAL] ' + res.error);
    setSerialBadge('error', 'SERIAL');
    setStatusBar(null, null, 'ERROR');
  }
});

dom.btnSerialDisc.addEventListener('click', async () => {
  if (!window.api) return appendError('No IPC bridge');
  const res = await window.api.invoke('serial-disconnect');
  state.serialConnected = false;
  setSerialBadge('', 'SERIAL');
  setStatusBar(null, null, 'DISCONNECTED');
  appendInfo('[SERIAL] Disconnected: ' + (res.message || ''));
});

dom.btnScan.addEventListener('click', async () => {
  if (!window.api) return appendError('No IPC bridge');
  appendInfo('[SERIAL] Scanning ports…');

  // Bug 5c fix: list all ports and populate the dropdown
  const res = await window.api.invoke('serial-list-ports');
  if (res.success && Array.isArray(res.ports)) {
    // Clear existing options (keep Auto-detect)
    while (dom.selPort.options.length > 1) dom.selPort.remove(1);

    if (res.ports.length === 0) {
      appendWarn('[SERIAL] No serial ports found.');
    } else {
      res.ports.forEach(p => {
        const opt = document.createElement('option');
        opt.value = p.path;
        opt.textContent = p.manufacturer
          ? `${p.path}  [${p.manufacturer}]`
          : p.path;
        dom.selPort.appendChild(opt);
      });
      appendInfo(`[SERIAL] Found ${res.ports.length} port(s): ` +
        res.ports.map(p => p.path).join(', '));
    }
  } else {
    appendError('[SERIAL] Port scan failed: ' + (res.error || 'unknown error'));
  }

  // Also trigger the internal better-port scan
  window.api.invoke('serial-scan-ports').catch(() => {});
});

// ── WebSocket (renderer-side, direct) ────────────────────────────────────────
function connectWebSocket(url, token) {
  if (state.ws) {
    state.ws.close();
    state.ws = null;
  }

  appendInfo(`[WS] Connecting to ${url}…`);
  setWsBadge('connecting', 'WS');

  let ws;
  try {
    ws = new WebSocket(url);
  } catch (e) {
    appendError('[WS] Invalid URL: ' + e.message);
    setWsBadge('error', 'WS');
    return;
  }

  ws.onopen = () => {
    state.wsConnected = true;
    setWsBadge('connected', 'WS');
    appendInfo('[WS] Connected');

    // Send handshake if token provided
    if (token) {
      ws.send(JSON.stringify({ type: 'auth', token }));
    }
  };

  ws.onmessage = (evt) => {
    const raw = evt.data;

    // Try JSON envelope first (from webSocketCommunicator)
    try {
      const parsed = JSON.parse(raw);
      const inner = parsed.data || parsed.payload || parsed;

      // If envelope wraps a raw serial line
      if (typeof inner === 'string') {
        parseLine(inner, new Date().toLocaleTimeString());
        return;
      }
      // If it's a structured JSON object, show it as info
      appendLog(JSON.stringify(inner), new Date().toLocaleTimeString(), 'type-info');
    } catch (_) {
      // Plain text / CSV line
      parseLine(raw, new Date().toLocaleTimeString());
    }
  };

  ws.onerror = (e) => {
    appendError('[WS] Error — check URL and server');
    setWsBadge('error', 'WS');
  };

  ws.onclose = (e) => {
    state.wsConnected = false;
    setWsBadge('', 'WS');
    appendWarn(`[WS] Closed (code ${e.code})`);
  };

  state.ws = ws;
}

dom.btnWsConn.addEventListener('click', () => {
  const url   = dom.inpWsUrl.value.trim();
  const token = dom.inpWsToken.value.trim();
  if (!url) return appendError('[WS] Please enter a URL');
  connectWebSocket(url, token);
});

dom.btnWsDisc.addEventListener('click', () => {
  if (state.ws) {
    state.ws.close();
    state.ws = null;
  }
  state.wsConnected = false;
  setWsBadge('', 'WS');
  appendInfo('[WS] Disconnected by user');
});

// ── Connection drawer toggle ──────────────────────────────────────────────────
dom.btnMenu.addEventListener('click', () => {
  dom.drawer.classList.toggle('open');
});

// ── Log controls ─────────────────────────────────────────────────────────────
dom.btnPauseLog.addEventListener('click', () => {
  state.logPaused = !state.logPaused;
  dom.btnPauseLog.classList.toggle('paused', state.logPaused);
  dom.btnPauseLog.textContent = state.logPaused ? '▶ Resume' : '⏸ Pause';
});

dom.btnClearLog.addEventListener('click', () => {
  dom.logScroll.innerHTML = '';
});

// ── Data rate counter ─────────────────────────────────────────────────────────
state.rateInterval = setInterval(() => {
  dom.sbRateVal.textContent = state.lineCountBuffer + ' ln/s';
  state.lineCountBuffer = 0;
}, 1000);

// ── CSV Recording state machine ───────────────────────────────────────────────
const rec = {
  status:   'idle',   // 'idle' | 'recording' | 'paused'
  filename: null,
  lineCount: 0,
  startTime: null,
  elapsedTimer: null,
};

function formatElapsed(ms) {
  const s   = Math.floor(ms / 1000);
  const m   = Math.floor(s / 60);
  const h   = Math.floor(m / 60);
  const pad = n => String(n).padStart(2, '0');
  return `${pad(h)}:${pad(m % 60)}:${pad(s % 60)}`;
}

function startElapsedTimer() {
  stopElapsedTimer();
  rec.elapsedTimer = setInterval(() => {
    if (rec.startTime && dom.recElapsed) {
      dom.recElapsed.textContent = formatElapsed(Date.now() - rec.startTime);
    }
  }, 1000);
}

function stopElapsedTimer() {
  if (rec.elapsedTimer) { clearInterval(rec.elapsedTimer); rec.elapsedTimer = null; }
}

function updateRecUI() {
  const s   = rec.status;
  const cls = s === 'recording' ? 'recording' : s === 'paused' ? 'paused' : '';

  // ── topbar elements ────────────────────────────────────────────────────────
  if (dom.recDot)    { dom.recDot.className = 'rec-dot ' + cls; }
  if (dom.recStatus) { dom.recStatus.textContent = s.toUpperCase(); }

  if (dom.btnRecStart) {
    dom.btnRecStart.disabled = s === 'recording';
    dom.btnRecStart.classList.toggle('recording', s === 'recording');
    dom.btnRecStart.textContent = s === 'paused' ? '▶ RESUME' : '● REC';
  }
  if (dom.btnRecPause) {
    dom.btnRecPause.disabled = s === 'idle';
    dom.btnRecPause.textContent = s === 'paused' ? '▶' : '⏸';
  }
  if (dom.btnRecStop) { dom.btnRecStop.disabled = s === 'idle'; }

  // ── log-bar elements ───────────────────────────────────────────────────────
  if (dom.recDot2)    { dom.recDot2.className = 'rec-dot ' + cls; }
  if (dom.recStatus2) { dom.recStatus2.textContent = s.toUpperCase(); }
  if (dom.recFilename){ dom.recFilename.textContent = rec.filename || '—'; }
  if (dom.recLines)   { dom.recLines.textContent = rec.lineCount + ' lines'; }

  if (dom.btnRecStart2) {
    dom.btnRecStart2.disabled = s === 'recording';
    dom.btnRecStart2.classList.toggle('recording', s === 'recording');
    dom.btnRecStart2.textContent = s === 'paused' ? '▶ RESUME' : '● REC';
  }
  if (dom.btnRecPause2) {
    dom.btnRecPause2.disabled = s === 'idle';
    dom.btnRecPause2.textContent = s === 'paused' ? '▶' : '⏸';
  }
  if (dom.btnRecStop2) { dom.btnRecStop2.disabled = s === 'idle'; }
}

async function recStart() {
  if (!window.api) return appendError('[REC] No IPC bridge available');
  if (rec.status === 'paused') { return recResume(); }
  const res = await window.api.invoke('csv-start');
  if (res && res.success) {
    rec.status    = 'recording';
    rec.filename  = res.filename;
    rec.lineCount = 0;
    rec.startTime = Date.now();
    startElapsedTimer();
    updateRecUI();
    appendInfo(`[REC] ● Recording → ${res.filename}`);
  } else {
    appendError('[REC] ' + (res ? res.error : 'unknown error'));
  }
}

async function recPause() {
  if (!window.api) return;
  if (rec.status === 'paused') { return recResume(); }
  const res = await window.api.invoke('csv-pause');
  if (res && res.success) {
    rec.status = 'paused';
    stopElapsedTimer();
    updateRecUI();
    appendInfo('[REC] ⏸ Recording paused');
  }
}

async function recResume() {
  if (!window.api) return;
  const res = await window.api.invoke('csv-resume');
  if (res && res.success) {
    rec.status = 'recording';
    startElapsedTimer();
    updateRecUI();
    appendInfo('[REC] ▶ Recording resumed');
  }
}

async function recStop() {
  if (!window.api) return;
  const res = await window.api.invoke('csv-stop');
  if (res && res.success) {
    const dur = res.duration ? Math.round(res.duration / 1000) + 's' : '';
    appendInfo(`[REC] ■ Saved ${res.lineCount} lines → ${res.filename} ${dur}`);
    rec.status    = 'idle';
    rec.filename  = null;
    rec.lineCount = 0;
    rec.startTime = null;
    stopElapsedTimer();
    if (dom.recElapsed) dom.recElapsed.textContent = '00:00:00';
    updateRecUI();
  } else {
    appendError('[REC] Stop failed: ' + (res ? res.error : 'unknown'));
  }
}

function setupRecording() {
  // Wire topbar buttons
  if (dom.btnRecStart)  dom.btnRecStart.addEventListener('click',  recStart);
  if (dom.btnRecPause)  dom.btnRecPause.addEventListener('click',  recPause);
  if (dom.btnRecStop)   dom.btnRecStop.addEventListener('click',   recStop);
  // Wire log-bar buttons (mirror)
  if (dom.btnRecStart2) dom.btnRecStart2.addEventListener('click', recStart);
  if (dom.btnRecPause2) dom.btnRecPause2.addEventListener('click', recPause);
  if (dom.btnRecStop2)  dom.btnRecStop2.addEventListener('click',  recStop);
  // Initial UI state
  updateRecUI();
}

// ── Kick off ─────────────────────────────────────────────────────────────────
setupElectronIPC();
setupRecording();
appendInfo('NDIR Monitor ready. Open the connection panel (⚙) to connect.');

