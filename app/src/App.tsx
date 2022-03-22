import { useEffect } from 'react';
import { wsConnect } from './utils/ws';
// must be after wsConnect import to prevent ` Cannot access 'getUri' before initialization`
import { HttpMethod, useHttp } from './hooks';
import { appConstants as c } from './app/constants';

// ignore websocket.js:88 WebSocket connection to 'wss://10.8.100.32/websockets/?EIO=4&transport=websocket' failed: Error in connection establishment: net::ERR_CONNECTION_REFUSED
// process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

export const App = () => {
  useEffect(() => {
    wsConnect();
  }, []);
  const { sendRequest } = useHttp();
  const transformFunction = (data: { message: string }) => {
    console.log(data);
  }
  const errorFunction = (error: any) => {
    console.error(error);
  }
  const pingClickHandler = async () => {
    await sendRequest({
      url: 'https://localhost:8543/ping',
      debug: false
    }, transformFunction, errorFunction);
  }
  const wsEchoClickHandler = async () => {
    await sendRequest({
      url: 'https://localhost:8543/api/ws-echo',
      method: HttpMethod.POST,
      headers: c.VALUES.apiHeaders,
      body: { message: 'hi there' },
      debug: false
    }, transformFunction, errorFunction);
  }
  return (
    <>
      <div className="m-4 p-4 rounded-lg bg-slate-200 border-2 border-slate-300">
        <h1 className="text-2xl text-gray-700 font-bold uppercase">
          Actixweb4-Starter + CRA 5.0 + TailwindCss 3 + Typescript Starter
        </h1>
      </div>
      <div className="m-4">
        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded uppercase" onClick={pingClickHandler}>
          /ping
        </button>
        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded uppercase ml-4" onClick={wsEchoClickHandler}>
          /wsEcho
        </button>
      </div>
    </>

  );
};
