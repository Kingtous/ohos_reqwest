# Reqwest for OpenHarmony

## 简要介绍

本库将Rust下全能HTTP/HTTPS库带到OpenHarmony生态，提供一层封装调用Reqwest预编译库，减少业务适配负担。

SDK版本需求：API 9及以上。

> 注：如果使用`@ohos-rs/abort-controller`搭配完成promise取消，则需要使用API 11（该库使用了API 11 ESObject特性）。具体详见本根仓库，运行即可。


## 特性

- 支持忽略SSL校验。在自签证书场景下尤为重要。（实测httpclient、axiosforhttpclient貌似还无法完全跳过SSL证书校验，或出现SSL is null的问题，于是有了这个库）
- 使用Rust编写。
- 封装支持传入取消控制器（API 11），中断请求promise。（由`[@ohos-rs/abort-controller](https://ohos-rs.github.io/ecosystem/polyfill/abort-controller.html)`提供支持）
- 全架构支持（aarch64/arm/x86_64）。
- 自动转换返回类型。（二进制、纯文本、json）

## 安装方法

注：本库还在审核中，可先源码依赖。

本库使用`ohpm`即可安装：
```shell
ohpm install @kingtous/reqwest
```

## 使用示例

一个简单的网络请求例子如下：


```typescript
import reqwest from '@kingtous/reqwest';

reqwest.request(this.url, "GET", {
  ignoreSsl: this.ignore_ssl,
  noProxy: this.noProxy,
  responseType: "application/json",
  caCert: [
    {
      ty: ReqwestCertType.PEM,
      cert: `MXXXXXXXXXXXXXXX==` // 不需要---BEGIN CERT和---ENC CERT，否则会报错，添加不上
    }
  ],
  timeout: 5000
}, {
  signal: this.abortController.signal // 取消信号
}).then((resp) => {
  AlertDialog.show({
    title: "请求结果",
    message: `${resp}`
  });
}).catch((error) => {
  AlertDialog.show({
    title: "请求失败",
    message: `${error}`
  });
});
```

其中options定义为：
```typescript
interface ReqwestOptions {
  responseType?: string;
  // 连接超时，ms
  connectTimeout?: number;
  // 输出传输超时：ms
  timeout?: number;
  // CA证书数组
  caCert: ReqwestCert[];
  // headers
  headers: Record<string, string>;
  // 以下三个body，按需传一个即可。
  body?: string;
  formBody?: Record<string, string>;
  jsonBody?: Record<string, string>;
  // 不遵循系统代理
  noProxy: boolean;
  // 忽略SSL校验
  ignoreSsl: boolean;
}
```

返回格式：
```typescript
interface ReqwestResponse {
    statusCode: number,
    responseBody: Uint8Array | Object | String,
    url: String,
    responseHeaders: Record<string, string>,
}
```

## Roadmap

- [ ] 新增下载文件、上传文件。
- [x] 重构接口。
- [ ] ...

## 贡献代码

Reqwest库是一个庞大的全功能库，本ohos库开源，欢迎大家参与共建/PR。

开源地址：https://gitee.com/kingtous/ohos_reqwest
