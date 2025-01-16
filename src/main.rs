// 导入 anyhow 库中的 Context 和 Result，用于错误处理
use anyhow::{Context, Result};
// 导入 clap 库的 Parser trait，用于命令行参数解析
use clap::Parser;
// 导入标准库的文件系统模块
use std::fs;
// 导入 tiktoken_rs 库的 cl100k_base 编码器
use tiktoken_rs::cl100k_base;

// 使用 clap 的 Parser 派生宏，自动实现命令行参数解析
#[derive(Parser)]
// 设置命令行程序的基本信息（作者、版本等）
#[command(author, version, about, long_about = None)]
struct Args {
    // 定义输入文件路径参数
    /// Path to the input text file
    #[arg()]
    input_file: String,

    // 定义编码器类型参数，默认值为 cl100k_base
    /// Tiktoken encoding to use (currently only supports cl100k_base)
    #[arg(long, default_value = "cl100k_base")]
    encoding: String,
}

// 主函数，返回 Result 类型用于错误处理
fn main() -> Result<()> {
    // 解析命令行参数
    let args = Args::parse();

    // 读取文件内容，如果失败则返回带上下文的错误信息
    let text = fs::read_to_string(&args.input_file)
        .with_context(|| format!("无法读取文件 '{}'", args.input_file))?;

    // 检查编码器类型是否支持
    if args.encoding != "cl100k_base" {
        // 如果不是 cl100k_base，则返回错误
        anyhow::bail!("目前只支持 cl100k_base 编码器");
    }
    
    // 创建 cl100k_base 分词器实例
    let bpe = cl100k_base().unwrap();
    // 使用分词器对文本进行编码，包括特殊token
    let tokens = bpe.encode_with_special_tokens(&text);
    // 打印token总数
    println!("Token 总数: {}", tokens.len());

    // 返回成功
    Ok(())
}
