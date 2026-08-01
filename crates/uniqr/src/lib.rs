use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
// 创建错误类型别名
type MyResult<T> = Result<T, Box<dyn Error>>;

// Config
#[derive(Debug)]
pub struct Config {
    in_file: String,          // 输入文件
    out_file: Option<String>, // 输出文件
    count: bool,              // 是否需要进行计数
}
// 获取参数
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("kyousuke")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-")
                .num_args(1), // 创建了一个变量in_file, 值占位符为IN_FILE
                              // 默认值为标准输入，接受参数只能为1个
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
                .num_args(0..=1), // 创建了一个out_file变量，值占位符为OUT_FILE
                                  // 可以接受0个或者1个输入
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Show counts")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    Ok(Config {
        in_file: matches.get_one::<String>("in_file").cloned().unwrap(),
        out_file: matches.get_one::<String>("out_file").map(String::from),
        count: matches.get_flag("count"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?; // map_err如果作用值是err变体，执行闭包，是ok变体，直接返回绑定的值
    let mut line = String::new();
    let mut pre_line = String::new(); // 指向上一段的开头的第一个字符串
    let mut pre_count: u64 = 0; // 已经出现了多少的重复的字符串
    let mut out_file: Box<dyn Write> = match &config.out_file {
        None => Box::new(io::stdout()),
        Some(out_name) => Box::new(File::create(out_name)?),
    }; // io::stdout()是标准输出区
    // File::create(path)创建一个文件写句柄
    let mut print = |count: u64, text: &str| {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text).expect("Could not write to file");
            } else {
                write!(out_file, "{}", text).expect("Could not write to file");
            }
        }
    };
    loop {
        let bytes = file.read_line(&mut line)?;
        // read_line表示读取一行得到的字节数
        // if bytes == 0 {
        //     print!("{}{}", if config.count {format!("   {} ", pre_count)} else {"".to_string()}, pre_line);
        //     break;
        // }
        // // 如果当前重复段结束了
        // if pre_line != line {
        //     // 如果当前不是初始字符串
        //     if !pre_line.is_empty() {
        //         // 打印
        //         print!("{}{}", if config.count {format!("   {} ", pre_count)} else {"".to_string()}, pre_line);
        //     }
        //     // 更
        //     pre_line = line.clone();
        //     pre_count = 1;
        // } else {
        //     pre_count += 1;
        // }

        if bytes == 0 {
            break;
        }
        if line.trim_end() != pre_line.trim_end() {
            print(pre_count, &pre_line);
            pre_line = line.clone();
            pre_count = 0;
        }
        pre_count += 1;
        line.clear();
    }
    print(pre_count, &pre_line);
    Ok(())
}
