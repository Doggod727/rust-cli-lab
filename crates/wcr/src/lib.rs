use clap::{Arg, Command, ArgAction};
use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("kyousuke")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files")
                .num_args(0..)
                .default_value("-")
            // 1. 创建了一个files参数，help占位符叫FILES, help输出选项Input file.
            // 至少需要0个参数
            // 默认值为"-"
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Show line count")
                .action(ArgAction::SetTrue)
            // 创建了一个lines参数，短参数名为-l, 长参数名为-lines, help信息为show line count, 设置时就为真
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("Show word count")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Show byte count")
                .action(ArgAction::SetTrue)
                .conflicts_with("chars")
                 // 也可以设置default_value
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("Show char count")
                .action(ArgAction::SetTrue)
                .conflicts_with("bytes")
        )
        .get_matches();
    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");

    // 创建了一个临时切片，使用迭代器获取其切片的元素的引用
    // 应用all判断是否满足闭包条件
    // 如果是更改
    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }
    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), // 标准输入
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!("{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );
                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }

                // 打印
                // if config.lines {
                //     print!{"{:>8}", file_info.num_lines};
                // }
                // if config.words {
                //     print!{"{:>8}", file_info.num_words};
                // }
                // if config.bytes {
                //     print!{"{:>8}", file_info.num_bytes};
                // } else if config.chars {
                //     print!{"{:>8}", file_info.num_chars};
                // }
                // println!(" {}", filename);
            }
        }
    }
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
        );
    }
    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    // 使用read_line方法，进行append
    let mut buffer = String::new();
    loop {
        buffer.clear(); // 清空，防止上一次的影响
        let bytes = file.read_line(&mut buffer)?; // bytes表示当前读取了多少字节，buffer存储字符串，包含了换行符
        // 如果EOF
        if bytes == 0 {
            break;
        }
        // 如果没有，说明读到了1行
        num_bytes += bytes; // 读取到的字节数吗
        num_lines += 1; // 读取到的行数
        num_chars += buffer.chars().count();
        num_words += buffer.split_whitespace().count();
    }
    Ok(FileInfo{
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo, format_field};
    use std::io::Cursor; // 模拟文件句柄， Cursor用于内存缓冲区任何实现了AsRef<[u8]>的类型，所以能够实现
    // Read Write， 从而让这些缓冲区可以用于任何实际执行I/O的读取器和写入器
    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text)); // 创建一个Cursor模拟文件句柄，位于内存缓冲区
        // 内存缓冲区存储的对象时text
        assert!(info.is_ok()); //
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(4, true), "       4");
    }
}