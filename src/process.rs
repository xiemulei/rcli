use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    // date of birth
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

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
/// 本函数首先通过`input`路径创建一个CSV文件的读取器，将CSV文件中的每行数据反序列化为`Player`结构体，
/// 然后将所有`Player`记录存储在向量`ret`中。之后，使用`serde_json`库将`ret`向量序列化为格式化的JSON字符串，
/// 并将该字符串写入由`output`指定路径的文件中。整个过程中，任意出现的错误都会导致函数立即终止并返回错误。
pub fn process_csv(input: &str, output: &str) -> Result<()> {
    // 创建CSV文件读取器
    let mut reader = Reader::from_path(input)?;
    // 初始化用于存储记录的向量，预设容量为128
    let mut ret = Vec::with_capacity(128);

    // 遍历CSV文件中的每一行记录，将其反序列化为Player对象并存入向量
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }

    // 将向量中的记录序列化为格式化的JSON字符串
    let json = serde_json::to_string_pretty(&ret)?;
    // 将JSON字符串写入输出文件
    fs::write(output, json)?;

    Ok(())
}
