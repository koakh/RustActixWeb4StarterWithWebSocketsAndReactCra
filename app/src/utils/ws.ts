import { IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket';
import { appConstants as c } from '../app/constants';
import { MessageToClientType } from '../types';
import { log } from './main';

export const wsConnect = (
) => {
  // prepare wsUri
  const client = new W3CWebSocket(c.VALUES.apiUrlWs);
  log(`wsUri: ${c.VALUES.apiUrlWs}`);

  client.onopen = () => {
    log('WebSocket Client Connected');
    const sendNumber = () => {
      if (client.readyState === client.OPEN) {
        var number = Math.round(Math.random() * 0xFFFFFF);
        client.send(number.toString());
        setTimeout(sendNumber, 1000);
      }
    }
    sendNumber();
  };
  // client.close = () => {
  //   log('WebSocket Client Disconnected');
  // };
  client.onclose = (error) => {
    log(`Socket is closed. Reconnect will be attempted in ${c.VALUES.wsReconnectTimeout} ${c.I18N.milliSeconds}.`, error.reason);
    setTimeout(() => {
      // call itSelf
      wsConnect();
    }, c.VALUES.wsReconnectTimeout);
  };
  client.onerror = (error) => {
    log('Connection Error', error.message);
  };
  client.onmessage = (message: IMessageEvent) => {
    const { msg_type: msgType, data }: { msg_type: String, data: any } = JSON.parse(message.data.toString());
    if (data) {
      // log(`msgType: [${msgType}]`);
      switch (msgType) {
        case MessageToClientType.Echo:
          log(`data: [${JSON.stringify(data, undefined, 0)}]`);
          break;
        default:
          // reach here in heartBeat
          // log(`unknown msgType: '${msgType}'`);
          break;
      };
    }
    // debug block
    // if (typeof message.data === 'string') {
    //   log(`Received string: ${message.data}`);
    // } else if (typeof message.data === 'object') {
    //   log(`Received object: ${JSON.stringify(message.data, undefined, 2)}`);
    // } else {
    //    log('Received other:', message.data);
    // };
  };
}
