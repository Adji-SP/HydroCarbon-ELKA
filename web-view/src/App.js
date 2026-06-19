import React from 'react';
import { useState, useEffect } from 'react';

function App() {
  const [data, setData] = useState({
    fm1FlowRate: 0,
    fm1Volume: 0,
    fm2FlowRate: 0,
    fm2Volume: 0
  });

  useEffect(() => {
    // You can reuse your API endpoints here
    const fetchData = async () => {
      try {
        const response = await fetch('http://localhost:3001/api/sensor-data');
        const result = await response.json();
        if (result.success) {
          setData(result.data);
        }
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    // Poll data every 5 seconds
    const interval = setInterval(fetchData, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <h1>Flow Meter System - Web Version</h1>
      </header>
      <main>
        <div className="status-cards">
          <div className="status-card">
            <h3>FM1 Flow Rate</h3>
            <p>{data.fm1FlowRate} L/min</p>
          </div>
          <div className="status-card">
            <h3>FM1 Volume</h3>
            <p>{data.fm1Volume} L</p>
          </div>
          <div className="status-card">
            <h3>FM2 Flow Rate</h3>
            <p>{data.fm2FlowRate} L/min</p>
          </div>
          <div className="status-card">
            <h3>FM2 Volume</h3>
            <p>{data.fm2Volume} L</p>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
