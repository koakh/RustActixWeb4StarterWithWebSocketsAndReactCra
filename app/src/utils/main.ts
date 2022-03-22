import { appConstants as c } from '../app/constants';

export const log = (message: any, context?: string) => {
  const showDebugInConsoleLog = (c.VALUES.showDebugInConsoleLog) ? true : false;
  if (showDebugInConsoleLog) {
    let outContext;
    let outMessage = message;
    if (typeof message === 'object') {
      outMessage = JSON.stringify(outMessage, undefined, 2);
    }
    outContext = context ? `[${context}] : ` : '';
    console.log(`${outContext}${outMessage}`);
  }
}

export const getUriHost = (): string => {
  // log(`PORT: ${process.env.REACT_APP_PORT} && REACT_APP_PORT_WS: ${process.env.REACT_APP_PORT_WS}`);
  return (process.env.REACT_APP_HOST_WS && process.env.REACT_APP_PORT_WS)
    ? `${process.env.REACT_APP_HOST_WS}:${process.env.REACT_APP_PORT_WS}`
    : window.location.host;
};

/**
 * get api uri
 * @returns 
 */
export const getUri = (): string => {
  // PUBLIC_URL comes from default package.json `"homepage":"/update"`
  return `https://${getUriHost()}${process.env.PUBLIC_URL}`
};

/**
 * get base uri ex https://c3edu.online
 * @returns 
 */
export const getUriBase = (): string => {
  const uri = getUri().split(':');
  // must replace both if exists
  return `${uri[0]}:${uri[1]}`
    .replace(`${process.env.PUBLIC_URL}`, '')
    .replace(`/api`, '');
};

/**
 * helper function to get websocket uri
 * with this gives browser.js:25 WebSocket connection to 'ws://127.0.0.1:8080/ws/' failed: 
 * const client = new W3CWebSocket('ws://127.0.0.1:8080/ws/', 'echo-protocol');
 */
export const getWsUri = (): string => {
  // eslint-disable-next-line no-mixed-operators
  return (window.location.protocol === 'https:' && 'wss://' || 'ws://') + getUriHost() + '/ws/';
};

export const getQueryVariable = (variable: string) => {
  const query = window.location.search.substring(1);
  //"app=article&act=news_content&aid=160990"
  // log(query);
  const vars = query.split("&");
  //[ 'app=article', 'act=news_content', 'aid=160990' ]
  // log(vars);
  for (let i = 0; i < vars.length; i++) {
    const pair = vars[i].split("=");
    //[ 'app', 'article' ][ 'act', 'news_content' ][ 'aid', '160990' ] 
    // log(pair);
    if (pair[0] === variable) { return pair[1]; }
  }
  return (false);
};

// used in response request or in webSocket events, ex when we lost connection with c3-updater and get into final update process
export const redirectTo = (uri: string, timeOut: number = 1000) => {
  log(`redirect to page in ${timeOut}${c.I18N.milliSeconds}`);
  setTimeout(() => {
    window.location.href = uri;
  }, timeOut);
};
