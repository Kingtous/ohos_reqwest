import { ReqwestOptions, ReqwestCertType, ReqwestCert, MiscOptions, ReqwestResponse } from "./src/main/ets/types/options";

export interface ReqwestStatic {
  request(url: string, method: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse>;

  get(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse>;

  post(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse>;

  delete_(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse>;

  patch(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse>;

  put(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<ReqwestResponse>;
}

declare const reqwest: ReqwestStatic;

export default reqwest;

export {
  ReqwestCertType, ReqwestCert, ReqwestOptions, MiscOptions
}