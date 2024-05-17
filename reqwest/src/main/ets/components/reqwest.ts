import ffi from 'libreqwest_native.so';

async function safeRequest(url: string, headers: Record<string, string>, method: string, body: string, ignoreSsl: boolean): Promise<string> {
  return ffi.safeRequest(url, headers, method, body, ignoreSsl);
}

export {
  safeRequest
}

