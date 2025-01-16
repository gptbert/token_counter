# Token Counter

一个简单的命令行工具，用于计算文本文件中的 token 数量。目前支持使用 OpenAI 的 cl100k_base 编码器进行 token 计算。

## 功能特点

- 支持读取文本文件
- 使用 tiktoken_rs 库进行 token 计算
- 支持 cl100k_base 编码器（与 GPT-3.5/GPT-4 使用的编码器相同）
- 提供清晰的命令行界面

## 安装

确保您的系统已安装 Rust 工具链，然后克隆本仓库并编译：

```bash
git clone [repository-url]
cd token_counter
cargo build --release
```

## 使用方法

基本用法：

```bash
token_counter <input_file> [--encoding cl100k_base]
```

参数说明：
- `input_file`：必需参数，指定要计算 token 的文本文件路径
- `--encoding`：可选参数，指定使用的编码器类型（当前仅支持 cl100k_base）

示例：

```bash
token_counter input.txt
token_counter input.txt --encoding cl100k_base
```

## 依赖库

- anyhow：错误处理
- clap：命令行参数解析
- tiktoken_rs：OpenAI token 计算库

## 注意事项

- 目前仅支持 cl100k_base 编码器
- 确保输入文件使用正确的文本编码（推荐 UTF-8）

## License

本项目采用 Apache License 2.0 协议。

Copyright 2024 Token Counter Contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.