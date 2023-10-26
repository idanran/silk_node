# Silk Node

[![npm](https://img.shields.io/npm/v/@silk_node/core?style=flat-square)](https://www.npmjs.com/package/@silk_node/core)

本项目已不再维护，请迁移至 [silk-wasm](https://github.com/idanran/silk-wasm)

## API
```ts
function encode(input: string, output: string, sampleRate: number): void
```

input 和 output 为文件地址，samplingRate 为采样率。

input 须为 pcm 文件。

## Support Platforms

* x86_64-apple-darwin
* x86_64-pc-windows-msvc
* x86_64-unknown-linux-gnu

