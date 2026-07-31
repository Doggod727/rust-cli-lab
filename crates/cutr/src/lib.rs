use clap::{Arg, ArgGroup, Command};
use std::error::Error; // 导入Error trait
use std::io::{self, BufRead, BufReader};
use std::ops::Range;
use crate::Extract::*;
use std::num::NonZeroUsize;
use regex::Regex;
use std::fs::File;
use csv::{StringRecord, ReaderBuilder, WriterBuilder}; // 解析csv文件的库
// 自定义Result类型
type MyResult<T> = Result<T, Box<dyn Error>>;
// 自定义一个PositionList类型，用来表示正整数值的范围
type PositionList = Vec<Range<usize>>; // 表示正整数值的范围


#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}
// 用来表示指定的类别和范围
// Config结构体
#[derive(Debug)]
pub struct Config {
    files : Vec<String>, // 输入文件列表
    delimiter: u8, // 指定分割符，默认时是制表符分隔
    extract: Extract, // 读入的指定提取范围, 必须指定三者中的一个，并且只能指定一个
    // 范围选择可以为单个数字或者2-4这样的范围
}
// 我们的挑战程序按照给定的顺序进行选择
// get_args()方法

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("cutr")
        .author("kyousuke")
        .about("Rust cut")
        .version("0.1.0")
        .arg(
            // 参数1，输入的文件列表
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
            // 创建了一个叫做files的参数，值占位名称为FILE
            // 可以接受0个或多个输入
            // 当输入是0个时，默认值为"-"
        )
        .arg(
            // 选项b，选择的字节
            Arg::new("bytes")
                .short('b')
                .long("bytes")
                .value_name("BYTES")
                .help("Selected bytes")
        )
        .arg(
            Arg::new("chars")
                .short('c')
                .long("chars")
                .value_name("CHARS")
                .help("Selected characters")

        )
        .arg(
            Arg::new("fields")
                .short('f')
                .long("fields")
                .value_name("FIELDS")
                .help("Selected fields")
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delim")
                .value_name("DELIMITER")
                .help("Field delimiter")
                .default_value("\t")
                .requires("fields"), // 必须使用了-f参数才能调用
        )
        .group(
            // 添加ArgGroup冲突方式
            ArgGroup::new("mode")
                .args(["chars", "bytes", "fields"])
                .required(true), // 如果不加required表示三者最多只能出现其中的一个
                // 加了就说明必须出现其中的一个
                // 如果不适用ArgGroup
            // 注意，conflicts检验优先级高于required
            // 并且conflict冲突检验是双向的
        )
        .get_matches();
    // 获取extract
    let bytes = matches
        .get_one::<String>("bytes")
        .map(|s| parse_pos(s))
        .transpose()?;
    let chars = matches
        .get_one::<String>("chars")
        .map(|s| parse_pos(s))
        .transpose()?;
    // &String 隐式转换为&str, 只针对于函数调用点，对于当前闭包不会强制转换
    let fields = matches
        .get_one::<String>("fields")
        .map(|s| parse_pos(s))
        .transpose()?;
    let extract = if let Some(field_pos) = fields {
        Fields(field_pos)
    } else if let Some(byte_pos) = bytes {
        Bytes(byte_pos)
    } else if let Some(char_pos) = chars {
        Chars(char_pos)
    } else {
        return Err(From::from("Must have --fields, --bytes, or --chars"));
    };
    // 谁非空就是谁
    // let extract = if let Some(bytes) = bytes {
    //     Bytes(parse_pos(bytes)?)
    // } else if let Some(chars) = chars {
    //     Chars(parse_pos(chars)?)
    // } else if let Some(fields) = fields {
    //     Fields(parse_pos(fields)?)
    // } else {
    //     unreachable!()
    // };
    let delimiter = matches.get_one::<String>("delimiter").unwrap();
    let delim_bytes = delimiter.as_bytes();
    if delim_bytes.len() != 1 {
        return Err(From::from(format!(
            "--delim \"{}\" must be a single byte",
            delimiter
        )))
    }
    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        delimiter: *delim_bytes.first().unwrap(),
        extract
    })
}

fn parse_index(input: &str) -> Result<usize, String> {
    let value_error = || format!("illegal list value: \"{}\"", input);
    input
        .starts_with('+')
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
            .map_err(|_| value_error())
        })
    // starts_with判断input是否由"+"开头，如果是，调用闭包，返回一个Option<Result<usize, String>>的Some变体
    // 否则返回None变体
    // 如果是None变体，将input进行划分为若干非0的值，将每一个值
}
// 解析range值
fn parse_pos(range: &str) -> MyResult<PositionList> {
    // 表示(\d+)表示一个或者多个数字
    let range_re = Regex::new(r"^(\d+)-(\d+)$")?;
    range.split(',')
        .into_iter()
        .map(|val| {
            parse_index(val).map(|n| n..n+1).or_else(
                |e| {
                    // 如果能够和正则表达式匹配，那么使用captures方法来提取数字，并且从1开始索引
                    range_re.captures(val).ok_or(e).and_then(
                        |captures| {
                            let n1 = parse_index(&captures[1])?;
                            let n2 = parse_index(&captures[2])?;
                            if n1 >= n2 {
                                return Err(format!(
                                    "First number in range ({}) \
                                     must be lower than second number ({})",
                                    n1 + 1, n2 + 1
                                ));
                            }
                            Ok(n1..n2+1)
                        }
                    )
                }
            )
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)

}
// run方法
// 夺取读取的参数结构体的所有权，成功返回Ok(())，失败返回任何实现了Error trait的类型
pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        // 获取文件句柄
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => match &config.extract {
                Fields(field_pos) => {
                    let mut reader = ReaderBuilder::new()
                        .delimiter(config.delimiter)
                        .has_headers(false)
                        .from_reader(file);
                    // 创建一个csv reader，用config.delimiter作为分隔符
                    // 认为不存在column列名称
                    // 从file读取
                    let mut wtr = WriterBuilder::new()
                        .delimiter(config.delimiter)
                        .from_writer(io::stdout());
                    // 创建一个csv writer，使用config.delimiter作为分隔符
                    // 写入到io::stdout()

                    for record in reader.records() {
                        // 开始读取每一个记录
                        let record = record?;
                        wtr.write_record(extract_fields(
                            &record, field_pos
                        ))?;
                    }
                    // 遍历每一行作为一个记录
                    // 解析出StringRecord
                    // 然后调用wtr写入到标准输出
                    // 写入的东西为提取的字段
                }
                Bytes(byte_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_bytes(&line?, byte_pos));
                    }
                }
                Chars(char_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_chars(&line?, char_pos));
                    }
                }
            }
        }

    }
    Ok(())
}

fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    // // 从每一行中提取出对应的范围的值
    // let mut result = vec![];
    // // 遍历char_pos, 然后将给定范围内的值直接获取
    // let chars: Vec<char> = line.chars().collect(); // 将line视为ASCII字符集，收集起来
    // for range in char_pos.iter().cloned() {
    //     for i in range {
    //         if let Some(val) = chars.get(i) {
    //             result.push(*val);
    //         }
    //     }
    // }
    // result.iter().collect()
    // let chars: Vec<_> = line.chars().collect();
    // let mut selected: Vec<char> = vec![];
    //
    // for range in char_pos.iter().cloned() {
    //     // extend将argument的迭代器的值添加到selected中去，
    //     // filter_map只返回闭包返回Some(val)的值
    //     selected.extend(range.filter_map(|i| chars.get(i)));
    // }
    //
    // selected.iter().collect()
    let chars: Vec<_> = line.chars().collect();
    // char_pos
    //     .iter()
    //     .cloned()
    //     .map(|range| range.filter_map(|i| chars.get(i)))
    //     .flatten()
    //     .collect()
    // map方法将每一个range对象应用闭包filter_map, 将chars.get(i)返回值为Some(val)的val收集起来，并得到一个迭代器
    // 于是我们得到一个迭代器的迭代器
    // 使用flatten方法，将迭代器解构为内部迭代器元素的迭代器
    // 也可以使用
    char_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        .collect()
}

fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = byte_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i).copied()))
        .collect();
    // range.filter_map(f)方法，对range应用闭包f，闭包f返回Option
    // 如果元素i返回Some(val)， 收集val，最后返回一个迭代器，指向所有收集的元素
    // flat_map(f)
    // 将迭代器元素应用闭包f，得到新的迭代器的迭代器
    // 然后应用flatten
    String::from_utf8_lossy(&selected).into_owned()
}

fn extract_fields<'a>(record: &'a StringRecord, field_pos: &[Range<usize>])  -> Vec<&'a str>{
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

#[cfg(test)]
mod unit_tests {
    use csv::StringRecord;
    use super::parse_pos;
    use super::extract_chars;
    use super::extract_bytes;
    use super::extract_fields;
    #[test]
    fn test_parse_pos() {
        // 单元测试LIST解析，数字可有前导0，不能包含任何非数字字符，数字范围可以用-表示多个数字和范围用逗号分割
        // 空字符串是错误的
        assert!(parse_pos("").is_err());

        // Zero is an Error
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"");

        // 前导符号为+，错误！
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
        "illegal list value: \"+1\"");

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
        "illegal list value: \"+1-2\"");

        // 非数字字符导致错误
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
                   "illegal list value: \"a\"");

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
                   "illegal list value: \"a\"");

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
                   "illegal list value: \"1-a\"");

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
                   "illegal list value: \"a-1\"");

        // 非法范围
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());

        // 必须是[)合法区间
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
        "First number in range (1) must be lower than second number (1)");

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
                   "First number in range (2) must be lower than second number (1)");

        // 正确示例
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[0..1]), "".to_string());
        assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
        assert_eq!(
            extract_chars("ábc", &[0..1, 1..2, 4..5]),
            "áb".to_string()
        );
    }
    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
    }


    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(
            extract_fields(&rec, &[0..1, 2..3]),
            &["Captain", "12345"]
        );
        assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    }
}