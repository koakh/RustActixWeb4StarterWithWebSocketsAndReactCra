import React from 'react';
import { LazyLog, ScrollFollow } from "react-lazylog";
import Line from 'react-lazylog/build/Line';
import './App.css';

process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';
// const url = 'ws://c3edu.online:8080/';
// const url = 'ws://c3edu.online:8080/ws/';
// NOTE: required last slash else it gives error `websocket.js:33 WebSocket connection to 'wss://c3edu.online:8470/ws' failed: Error during WebSocket handshake: Unexpected response code: 404`
const url = 'wss://c3edu.online:8470/ws/';
// let socket: any = null;

// Use defaultProps.style to set the style for an internal component
// TODO: this doesn do nothing?
Line.defaultProps.style = {
  color: 'green'
};

function App() {
  // return (
  //   <LazyLog url="http://example.log" />
  // );

  // return (
  //   <div style={{ height: 500, width: 902 }}>
  //     <LazyLog extraLines={1} enableSearch url={url} caseInsensitive />
  //   </div>
  // );

  return (
    <div>
      {/* <button
        style={{ marginBottom: 8, background: '#eee' }}
        onClick={() => socket && socket.send(JSON.stringify({ message: '[taskcluster 2018-11-14 21:08:32.452Z] Worker Group: us-east-1' }))}>
        ping
      </button> */}
      <div style={{ height: 600 }}>
        <ScrollFollow
          startFollowing={true}
          render={({ follow }) => (
            <LazyLog
              enableSearch
              caseInsensitive
              extraLines={1}
              url={url}
              stream
              follow={follow}
              websocket
              websocketOptions={{
                onOpen: (e: Event, socket: WebSocket) => {
                  // sock.send(JSON.stringify({ message: "Socket has been opened!" }));
                  console.log('socket onOpen', e);
                },
                onClose: (e: CloseEvent) => {
                  // sock.send(JSON.stringify({ message: "Socket has been closed!" }));
                  console.log('socket onClose', e);
                },
                onError: (e: Event) => {
                  // sock.send(JSON.stringify({ message: "Socket has errors!" }));
                  console.log('socket onError', e);
                },
                // object
                // formatMessage: (e: any) => JSON.parse(e).message
                // just line
                // formatMessage: (e: any) => e,
                formatMessage: (e: any) => JSON.parse(e)?.data?.message
              }}
            />
          )}
        />
      </div>
    </div>
  );
}

export default App;