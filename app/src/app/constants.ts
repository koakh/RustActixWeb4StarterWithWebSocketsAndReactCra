import { getUri, getWsUri } from "../utils";

const VALUES: { [key: string]: any } = {
  appName: 'actixweb4-starter',
  // used to add/remove path for ex for from redirect url
  c3UpdaterPath: process.env.PUBLIC_URL || '',
  apiUrl: getUri(),
  apiUrlWs: getWsUri(),
  apiHeaders: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${process.env.HTTP_SERVER_API_KEY}`,
  },
  showDebugInConsoleLog: (process.env.REACT_APP_SHOW_DEBUG_IN_CONSOLE_LOG || false),
  wsReconnectTimeout: 1500,
  showAlertMessageTimeout: 15000,
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
