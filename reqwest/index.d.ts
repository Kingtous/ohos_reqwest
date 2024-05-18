import { ReqwestOptions, ReqwestCertType, ReqwestCert } from "./src/main/ets/types/options";

export interface ReqwestStatic {
  request(url: string, method: string, options: ReqwestOptions): Promise<string>;
}

declare const reqwest: ReqwestStatic;

export default reqwest;

export {
  ReqwestCertType,
  ReqwestCert
}