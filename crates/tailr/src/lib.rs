use crate::TakeValue::*;
use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
type MyResult<T> = Result<T, Box<dyn Error>>;

// 只编译一次正则表达式
//static NUM_RE: OnceCell<Regex> = OnceCell::new();
#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,     // +0表示选择全部内容
    TakeNum(i64), // 存储其他所有输入值
}
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,         // 默认值为TakeNum(-10)表示最后10行
    bytes: Option<TakeValue>, // 可选的，用于指定可选的字节数
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("tailr")
        .version("0.1.0")
        .author("kyousuke")
        .about("Rust tail")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Suppress headers"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10")
                .num_args(1)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .num_args(1), // 如果出现了就要吃值
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        quiet: matches.get_flag("quiet"),
        lines: matches
            .get_one::<String>("lines")
            .map(|val| parse_num(val))
            .transpose()
            .map_err(|e| format!("illegal line count -- {}", e))?
            .unwrap(),
        bytes: matches
            .get_one::<String>("bytes")
            .map(|val| parse_num(val))
            .transpose()
            .map_err(|e| format!("illegal byte count -- {}", e))?,
    })
}

fn parse_num(val: &str) -> MyResult<TakeValue> {
    // 惰性求值静态变量
    // 第一次调用创建闭包获取值
    // 后续get
    // OneInstance方法
    // let num_re = NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());
    //
    // // ([+-])?是捕获组1
    // // (\d+)是捕获组2
    // match num_re.captures(val) {
    //     //caps是捕获组的内容，
    //     Some(caps) => {
    //         let sign = caps.get(1).map_or("-", |m| m.as_str()); // map_or将Some(val) 的val应用闭包返回闭包返沪值，None应用默认值应用闭包
    //         let num = format!("{}{}", sign, caps.get(2).unwrap().as_str()); // 获取带符号整数
    //         // 如果输入的是+0, val == 0, sign == "+", 分支1
    //         // 如果输入的是+K, val == K, sign == "+" 分支2
    //         // 如果输入的是-0, val == 0, sign == "-" 分支2
    //         // 如果输入的是-K, val == -K, sign == "-" 分支2
    //         if let Ok(val) = num.parse() {
    //             if sign == "+" && val == 0 {
    //                 Ok(PlusZero)
    //             } else {
    //                 Ok(TakeNum(val))
    //             }
    //         } else {
    //             Err((From::from(val)))
    //         }
    //     }
    //     _ => Err(From::from(val)),
    // }

    let signs: &[char] = &['+', '-']; // 创建一个字符串切片
    let res = val
        .starts_with(signs)
        .then(|| val.parse()) // 如果输入的val以+或者-开头，应用闭包获得一个Result<i64>类型
        .unwrap_or_else(|| val.parse().map(i64::wrapping_neg));

    match res {
        Ok(num) => {
            if num == 0 && val.starts_with("+") {
                Ok(PlusZero)
            } else {
                Ok(TakeNum(num))
            }
        }
        _ => Err(From::from(val)),
    }
    // 如果输入+0, res => Ok(0), 分支1
    // +k, res => Ok(K) 分支2
    // -0 || 0 res => Ok(0) 分支2
    // -K || K res => Ok(K) 分支2
}

fn count_lines_bytes(filename: &str) -> MyResult<(i64, i64)> {
    let mut total_lines = 0;
    let mut total_bytes = 0;
    let mut line = Vec::new(); // 缓冲区
    let mut reader = BufReader::new(File::open(&filename)?);
    loop {
        // 直接读取原始字节到vec中去，没有创建字符串的开销了
        let bytes = reader.read_until(b'\n', &mut line)?; // 换行符要计算到字节数中去，不能用line去忽略
        if bytes == 0 {
            break;
        }
        total_bytes += bytes;
        total_lines += 1;
        line.clear();
    }
    Ok((total_lines, total_bytes as i64))
}

fn print_lines(mut file: impl BufRead, num_lines: &TakeValue, total_lines: i64) -> MyResult<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
        let mut line_num = 0; // 当前遍历到的行数，start为打印起始行，如果line_num >= start就可以打印
        let mut buf = Vec::new(); // 读取字节缓冲区
        loop {
            let bytes_read = file.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }
            line_num += 1;
            buf.clear();
        }
    }
    Ok(())
}
fn print_bytes<T: Read + Seek>(
    mut file: T,
    num_bytes: &TakeValue,
    total_bytes: i64,
) -> MyResult<()> {
    if let Some(start) = get_start_index(num_bytes, total_bytes) {
        file.seek(SeekFrom::Start(start))?; // 将file的游标重定向到start个字节
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        if !buffer.is_empty() {
            print!("{}", String::from_utf8_lossy(&buffer));
        }
    }
    Ok(())
}
// 只是用于找到打印的起始位置，也就是用来解析
fn get_start_index(take_val: &TakeValue, total: i64) -> Option<u64> {
    match take_val {
        PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }
        TakeNum(num) => {
            if num == &0 || total == 0 || num > &total {
                None
            } else {
                let start = if num < &0 { total + num } else { num - 1 };
                Some(if start < 0 { 0 } else { start as u64 })
            }
        }
    }
}
pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match File::open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if !config.quiet && num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename,
                    )
                }
                let (total_lines, total_bytes) = count_lines_bytes(&filename)?;
                let file = BufReader::new(file);
                if let Some(num_bytes) = &config.bytes {
                    print_bytes(file, num_bytes, total_bytes)?
                } else {
                    print_lines(file, &config.lines, total_lines)?
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{TakeValue::*, count_lines_bytes, get_start_index, parse_num};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        let (lines, bytes) = res.unwrap();
        assert_eq!(lines, 1);
        assert_eq!(bytes, 24);

        let res = count_lines_bytes("tests/inputs/twelve.txt");
        assert!(res.is_ok());
        let (lines, bytes) = res.unwrap();
        assert_eq!(lines, 12);
        assert_eq!(bytes, 63);
    }

    #[test]
    fn test_get_start_index() {
        // +0 from an empty file (0 lines/bytes) returns None
        assert_eq!(get_start_index(&PlusZero, 0), None);

        // +0 from a nonempty file returns an index that
        // is one less than the number of lines/bytes
        assert_eq!(get_start_index(&PlusZero, 1), Some(0));

        // Taking 0 lines/bytes returns None
        assert_eq!(get_start_index(&TakeNum(0), 1), None);

        // Taking any lines/bytes from an empty file returns None
        assert_eq!(get_start_index(&TakeNum(1), 0), None);

        // Taking more lines/bytes than is available returns None
        assert_eq!(get_start_index(&TakeNum(2), 1), None);

        // When starting line/byte is less than total lines/bytes,
        // return one less than starting number
        assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
        assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
        assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

        // When starting line/byte is negative and less than total,
        // return total - start
        assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
        assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
        assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

        // When the starting line/byte is negative and more than the total,
        // return 0 to print the whole file
        assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
    }

    #[test]
    fn test_parse_num() {
        let res = parse_num("3"); // 字符串字面量本身就是 &str，不用分配
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        let res = parse_num("+3");
        assert_eq!(res.unwrap(), TakeNum(3));

        let res = parse_num("-3");
        assert_eq!(res.unwrap(), TakeNum(-3));

        let res = parse_num("0");
        assert_eq!(res.unwrap(), TakeNum(0));

        let res = parse_num("+0");
        assert_eq!(res.unwrap(), PlusZero);

        // 边界值需要临时 String，取引用即可
        let res = parse_num(&i64::MAX.to_string());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&(i64::MIN + 1).to_string());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&format!("+{}", i64::MAX));
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(&i64::MIN.to_string());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        let res = parse_num("3.14");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        let res = parse_num("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}
