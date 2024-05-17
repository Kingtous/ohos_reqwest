export interface ReqwestStatic {
  safeRequest(url: string, headers: Record<string, string>, method: string, body: string, ignoreSsl: boolean): Promise<string>;
}

declare const Reqwest: ReqwestStatic;

export default Reqwest;