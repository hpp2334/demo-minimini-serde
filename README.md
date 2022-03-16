# \[Demo\] minimini-serde

本 Demo 是一个简化版本的 [miniserde](https://github.com/dtolnay/miniserde)，主要用于学习 miniserde 的序列化与反序列 API。

Demo 在原本的基础上做出以下简化：

- 重点考虑实现序列化 API 为 `to_tokens`，反序列化 API 为 `from_tokens`；
- 只支持 `bool`, `i32`, `String` 与 struct；
- 针对 Token，而非字符串；
- 不考虑性能、合法性校验等；
