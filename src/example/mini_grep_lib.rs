use std::env::{self, Args};
use std::error::Error;
use std::fs;

// dyn 表示动态
pub fn get_file_content(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;

    // 获取小写
    let _res: bool = if config.is_big {
        println!("小写 {}", content.to_lowercase());
        false
    } else {
        println!("大写 {}", content.to_uppercase());
        true
    };

    println!("文件内容：\r\n{}", content);
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut res = Vec::new();

    // for line in content.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }

    // res

    // 第二种实现
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub struct Config {
    pub filename: String,
    pub query: String,
    pub is_big: bool,
}

impl Config {
    // 解析命令行参数
    // :[String] 为 字符串切片
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }

        // clone 会引发性能损耗
        // let query = args[1].clone();
        // let filename = args[2].clone();

        args.next();
        let query = match args.next() {
            Some(v) => v,
            None => return Err("参数缺少错误"),
        };

        let filename = match args.next() {
            Some(v) => v,
            None => return Err("参数缺少错误"),
        };

        // is_err 环境变量是否出现 出现了就是 true 否则就是错误 直接返回 false
        // 如何使用环境变量：IS_BIG=1 cargo run
        let is_big: bool = env::var("IS_BIG").is_err();

        Ok(Config {
            filename,
            query,
            is_big,
        })
    }
}
