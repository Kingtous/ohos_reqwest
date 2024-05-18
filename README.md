# Reqwest for OpenHarmony


## 简要介绍

本库将Rust下全能HTTP/HTTPS库带到OpenHarmony生态，提供一层封装调用Reqwest预编译库，减少业务适配负担。

SDK版本需求：API 9及以上。

> 注：如果使用`@ohos-rs/abort-controller`搭配完成promise取消，则需要使用API 11（该库使用了API 11 ESObject特性）。具体详见本根仓库，运行即可。


库在`reqwest_native`文件夹中，详细说明： [README.md](./reqwest/README.md)