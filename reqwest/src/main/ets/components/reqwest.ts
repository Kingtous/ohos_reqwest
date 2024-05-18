import ffi from 'libreqwest_native.so';
import { ReqwestCert, ReqwestOptions, ReqwestCertType } from '../types/options';


async function _request(url: string, method: string, options: string): Promise<string> {
  return ffi.request(url, method, options);
}

async function request(url: string, method: string, options: ReqwestOptions): Promise<string> {
  return _request(url, method, JSON.stringify(options));
}

export {
  request,
  ReqwestCert,
  ReqwestOptions,
  ReqwestCertType
}

