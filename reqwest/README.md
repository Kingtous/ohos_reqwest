# Reqwest for OpenHarmony

本库将Rust下全能HTTP/HTTPS库带到OpenHarmony生态。


## 特性

- 支持忽略SSL校验。在自签证书场景下尤为重要。（实测httpclient、axiosforhttpclient貌似还无法完全跳过SSL证书校验，或出现SSL is null的问题，于是有了这个库）
- 使用Rust编写。


## Roadmap

- [ ] 新增下载文件、上传文件。
- [ ] 重构接口。
- [ ] ...