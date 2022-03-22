import { useState, useCallback } from 'react';
import { log } from '../utils/main';

// NOTES
// use NODE_TLS_REJECT_UNAUTHORIZED="0" to ignore ssl certificates, else we get a 500 status code
// Links: https://javascript.info/fetch-api

export enum HttpMethod {
  GET = 'GET',
  HEAD = 'HEAD',
  POST = 'POST',
  PUT = 'PUT',
  DELETE = 'DELETE',
  CONNECT = 'CONNECT',
  OPTIONS = 'OPTIONS',
  TRACE = 'TRACE',
  PATCH = 'PATCH',
}

export interface RequestConfig {
  url: string,
  method?: HttpMethod,
  headers?: Headers,
  body?: any,
  keepAlive?: boolean,
  debug?: boolean,
}

export const useHttp = () => {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  // add generic type for applyData response
  const sendRequest = useCallback(async (requestConfig: RequestConfig, transformFunction: (data: any) => any, errorFunction?: (error: Error) => void) => {
    if (requestConfig.debug) {
      log(`requestConfig: [${JSON.stringify(requestConfig, undefined, 2)}]`);
    }
    setIsLoading(true);
    setError(null);
    try {
      const fetchResult = await fetch(requestConfig.url, {
        method: requestConfig.method ? requestConfig.method : HttpMethod.GET,
        headers: requestConfig.headers ? requestConfig.headers : {},
        body: requestConfig.body ? JSON.stringify(requestConfig.body) : undefined,
        keepalive: requestConfig.keepAlive || true,
      });

      if (!fetchResult.ok) {
        // there real trick to get message from error is fetchResult.json()
        const result = await fetchResult.json();
        // api will respond with a error message like { message: 'error info....' }
        throw new Error(result.message || 'Request failed!');
      }

      const data = await fetchResult.json();
      transformFunction(data);
    } catch (err: any) {
      setError(err.message || 'Something went wrong!');
      if (typeof errorFunction === 'function') {
        errorFunction(err);
      }
    }
    setIsLoading(false);
  }, []);

  return {
    isLoading,
    error,
    sendRequest,
  };
};
