# Reqwest for OpenHarmony

## 简要介绍

本库将Rust下全能HTTP/HTTPS库带到OpenHarmony生态，提供一层封装调用Reqwest预编译库，减少业务适配负担。


## 特性

- 支持忽略SSL校验。在自签证书场景下尤为重要。（实测httpclient、axiosforhttpclient貌似还无法完全跳过SSL证书校验，或出现SSL is null的问题，于是有了这个库）
- 使用Rust编写。
- 全架构支持（aarch64/arm/x86_64）。

## 安装方法

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


## Roadmap

- [ ] 新增下载文件、上传文件。
- [ ] 重构接口。
- [ ] ...

## 贡献代码

Reqwest库是一个庞大的全功能库，本ohos库开源，欢迎大家参与共建/PR。

开源地址：https://gitee.com/kingtous/ohos_reqwest
