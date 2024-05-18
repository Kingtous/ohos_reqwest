import ffi from 'libreqwest_native.so';
import { ReqwestCert, ReqwestOptions, ReqwestCertType, MiscOptions } from '../types/options';


async function _request(url: string, method: string, options: string): Promise<string> {
  return ffi.request(url, method, options);
}

async function get(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string> {
  return request(url, 'GET', options, miscOptions);
}

async function post(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string> {
  return request(url, 'POST', options, miscOptions);
}

async function put(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string> {
  return request(url, 'PUT', options, miscOptions);
}

/// delete 是 ArkTS关键字
async function delete_(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string> {
  return request(url, 'DELETE', options, miscOptions);
}

async function patch(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string> {
  return request(url, 'PATCH', options, miscOptions);
}


async function request(url: string, method: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string> {
  const signal = miscOptions?.signal;
  if (signal !== undefined) {
    return new Promise((resolve, reject) => {
      const innerPromise = _request(url, method, JSON.stringify(options));
      signal.addEventListener(('abort'), () => {
        reject("request aborted by signal.");
      });
      innerPromise.then((v) => {
        resolve(v);
      }, (reason) => {
        reject(reason);
      })
      return innerPromise;
    });
  } else {
    return _request(url, method, JSON.stringify(options));
  }
}

export {
  request, ReqwestCert, ReqwestOptions, ReqwestCertType, MiscOptions, get, post, patch, delete_, put
}

