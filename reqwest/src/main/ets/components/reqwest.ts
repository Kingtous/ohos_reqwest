import ffi from 'libreqwest_native.so';
import { ReqwestCert, ReqwestOptions, ReqwestCertType, MiscOptions, ReqwestResponse } from '../types/options';
import util from '@ohos.util';

const _base64Helper = new util.Base64Helper();
const _textDecoder = util.TextDecoder.create();

async function _request(url: string, method: string, options: string): Promise<ReqwestResponse> {
  return new Promise((resolve, reject) => {
    ffi.request(url, method, options).then(async (value) => {
      const json = JSON.parse(value);
      try {
        const resp = await _base64Helper.decode(json['response_body']);
        const statusCode: number = json['status_code'];
        const url: string = json['url'];
        const responseHeaders = json['response_headers'];
        try {
          const text = _textDecoder.decodeWithStream(resp);
          // 看看是不是json
          try {
            const bodyJson: object = JSON.parse(text);
            resolve({
              statusCode: statusCode,
              responseBody: bodyJson,
              responseHeaders: responseHeaders,
              url: url,
            });
          } catch (e) {
            resolve({
              statusCode: statusCode,
              responseBody: text,
              responseHeaders: responseHeaders,
              url: url,
            });
          }
        } catch (e) {
          // 不是text
          resolve({
            statusCode: statusCode,
            responseBody: resp,
            responseHeaders: responseHeaders,
            url: url,
          });
        }
      } catch (e) {
        reject(`base64 decode body error: ${e}`);
      }
    }, (reason) => {
      reject(reason);
    })
  });
}

async function get(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse> {
  return request(url, 'GET', options, miscOptions);
}

async function post(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse> {
  return request(url, 'POST', options, miscOptions);
}

async function put(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse> {
  return request(url, 'PUT', options, miscOptions);
}

/// delete 是 ArkTS关键字
async function delete_(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse> {
  return request(url, 'DELETE', options, miscOptions);
}

async function patch(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse> {
  return request(url, 'PATCH', options, miscOptions);
}


async function request(url: string, method: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse> {
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

