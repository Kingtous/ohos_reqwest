import { ReqwestOptions, ReqwestCertType, ReqwestCert, MiscOptions } from "./src/main/ets/types/options";

export interface ReqwestStatic {
  request(url: string, method: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string>;

  get(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string>;

  post(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string>;

  delete_(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string>;

  patch(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string>;

  put(url: string, options: ReqwestOptions, miscOptions?: MiscOptions): Promise<string>;
}

declare const reqwest: ReqwestStatic;

export default reqwest;

export {
  ReqwestCertType, ReqwestCert, ReqwestOptions, MiscOptions
}