import { getUri as getApiUri, getUriHost, getWsUri } from "../utils/main";

const VALUES: { [key: string]: any } = {
  appName: 'actixweb4-starter',
  apiUrl: getApiUri(),
  apiUrlWs: getWsUri(),
  apiHeaders: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${process.env.REACT_APP_HTTP_SERVER_API_KEY}`,
  },
  showDebugInConsoleLog: (process.env.REACT_APP_SHOW_DEBUG_IN_CONSOLE_LOG || false),
  wsReconnectTimeout: 1500
};

const I18N: { [key: string]: string } = {
  // keywords
  undefined: 'Undefined',
  error: 'Error',
};

export const appConstants = {
  VALUES,
  I18N,
};
