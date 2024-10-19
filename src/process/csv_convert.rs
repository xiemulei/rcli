use std::fs;

use anyhow::Result;
use csv::Reader;
use serde_json::Value;

use crate::cli::OutputFormat;

/// 从CSV文件读取数据，处理后以JSON格式写入到输出文件
///
/// # 参数
/// - `input`: 输入CSV文件的路径
/// - `output`: 输出JSON文件的路径
///
/// # 返回值
/// - `Ok(())` 表示操作成功
/// - `Err(Error)` 表示操作中遇到错误，返回的是错误信息
///
/// # 说明
/// 本函数首先通过`input`路径创建一个CSV文件的读取器，将CSV文件中的每行数据反序列化为 serde Value，
/// 然后将所有记录存储在向量`ret`中。之后，使用`serde_json`库将`ret`向量序列化为格式化的JSON字符串，
/// 并将该字符串写入由`output`指定路径的文件中。整个过程中，任意出现的错误都会导致函数立即终止并返回错误。
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // 创建CSV文件读取器
    let mut reader = Reader::from_path(input)?;
    // 初始化用于存储记录的向量，预设容量为128
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        // 遍历CSV文件中的每一行记录
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并成一个元组的迭代器 [(header, record), ...]
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    // 将字符串写入输出文件
    fs::write(output, content)?;

    Ok(())
}
