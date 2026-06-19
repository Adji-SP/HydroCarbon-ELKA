const { contextBridge, ipcRenderer } = require('electron');

const validInvokeChannels = new Set([
  'get-data-by-filters',
  'delete-data',
  'insert-data',
  'update-data',
  'serial-force-reconnect',
  'serial-disconnect',
  'serial-scan-ports',
  'serial-list-ports',
  'serial-toggle-dynamic-switching',
  'serial-get-status',
  'serial-send-data',
  'csv-start',
  'csv-pause',
  'csv-resume',
  'csv-stop',
  'csv-get-status',
]);

const validReceiveChannels = new Set([
  'serial-data-received',
  'serial-port-status',
  'serial-port-error',
  'serial-connection-lost',
  'serial-reconnect-status',
  'serial-port-switched',
  'database-insert-success',
  'serial-data-sent',
]);

contextBridge.exposeInMainWorld('api', {
  invoke: (channel, ...args) => {
    if (!validInvokeChannels.has(channel)) {
      console.warn(`Invalid invoke channel: ${channel}`);
      return Promise.reject(new Error(`Invalid channel: ${channel}`));
    }
    return ipcRenderer.invoke(channel, ...args);
  },

  receive: (channel, callback) => {
    if (!validReceiveChannels.has(channel)) {
      console.warn(`Invalid receive channel: ${channel}`);
      return;
    }
    ipcRenderer.on(channel, (_, ...args) => callback(...args));
  },

  removeAllListeners: (channel) => {
    ipcRenderer.removeAllListeners(channel);
  },

  // Database convenience methods
  getDataByFilters: (table, filters, options) => ipcRenderer.invoke('get-data-by-filters', table, filters, options),
  deleteData: (table, whereClause, whereParams) => ipcRenderer.invoke('delete-data', table, whereClause, whereParams),
  insertData: (table, data) => ipcRenderer.invoke('insert-data', table, data),
  updateData: (table, data, whereClause, whereParams) => ipcRenderer.invoke('update-data', table, data, whereClause, whereParams),

  // Serial convenience methods
  getSerialStatus: () => ipcRenderer.invoke('serial-get-status'),
  forceReconnect: () => ipcRenderer.invoke('serial-force-reconnect'),
  disconnect: () => ipcRenderer.invoke('serial-disconnect'),
  scanPorts: () => ipcRenderer.invoke('serial-scan-ports'),
  setDynamicSwitching: (enabled) => ipcRenderer.invoke('serial-toggle-dynamic-switching', enabled),
  sendData: (data) => ipcRenderer.invoke('serial-send-data', data),
});