enum ReqwestCertType {
  PEM = "pem",
  DER = "der"
}

interface ReqwestCert {
  ty: ReqwestCertType;
  // 证书字符串
  cert: string;
}

interface ReqwestOptions {
  responseType?: string;
  // 连接超时，ms
  connectTimeout?: number;
  // 输出传输超时：ms
  timeout?: number;
  // CA证书数组
  caCert?: ReqwestCert[];
  // headers
  headers?: Record<string, string>;
  // 以下三个body，按需传一个即可。传多个的话，按以下顺序匹配到一个不为null就停止。
  body?: string;
  formBody?: Record<string, string>;
  jsonBody?: Record<string, string>;
  // 不遵循系统代理
  noProxy?: boolean;
  // 忽略SSL校验
  ignoreSsl?: boolean;
  // 强制使用rustls作为openssl实现。目前无需关注。
  forceRustlsSsl?: boolean;
}

export {
  ReqwestCertType,
  ReqwestCert,
  ReqwestOptions
}