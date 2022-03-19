import logo from './logo.svg';
import './App.css';
import { useEffect } from 'react';
import { wsConnect } from './utils/ws';

// ignore websocket.js:88 WebSocket connection to 'wss://10.8.100.32/websockets/?EIO=4&transport=websocket' failed: Error in connection establishment: net::ERR_CONNECTION_REFUSED
// process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

export const App = () => {
  useEffect(() => {
    wsConnect();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
};
