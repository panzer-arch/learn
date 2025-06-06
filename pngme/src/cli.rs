use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pngme")]
#[command(version = "1.0")]
#[command(about = "add message into png file", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// png文件加密
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
        output_file: Option<String>,
    },
    /// png文件解密
    /// 解析指定类型的chunk
    Decode {
        file_path: String,
        chunk_type: String,
    },
    /// 删除指定类型的chunk
    Remove {
        file_path: String,
        chunk_type: String,
    },
    /// 打印指定png
    Print { file_path: String },
}
