# Silk Node

[![npm](https://img.shields.io/npm/v/@silk_node/core?style=flat-square)](https://www.npmjs.com/package/@silk_node/core)

QQ/微信语音编码器

## API
```ts
function encode(input: string, output: string, sampleRate: number): void
```

input 和 output 为文件地址，samplingRate 为采样率。

input 须为 pcm 文件。

