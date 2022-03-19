import { useEffect } from 'react';
import { wsConnect } from './utils/ws';

// ignore websocket.js:88 WebSocket connection to 'wss://10.8.100.32/websockets/?EIO=4&transport=websocket' failed: Error in connection establishment: net::ERR_CONNECTION_REFUSED
// process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

export const App = () => {
  useEffect(() => {
    wsConnect();
  }, []);

  return (
    <div className="m-4 p-4 rounded-lg bg-slate-200 border-2 border-slate-300">
      <h1 className="text-2xl text-gray-700 font-bold uppercase">
        Actixweb4-Starter + CRA 5.0 + TailwindCss 3 + Typescript Starter
      </h1>
    </div>
  );
};
