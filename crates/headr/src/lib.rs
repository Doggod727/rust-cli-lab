use clap::builder::RangedU64ValueParser;
use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
// 自定义错误类型别名
// T指代的是Ok变体的类型
type MyResult<T> = Result<T, Box<dyn Error>>;
// Config结构体，用来指定接受的参数

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,   // 文件名
    lines: usize,         // -n指定的行数，默认是10
    bytes: Option<usize>, // -c指定的字节数，没有输入时不处理
}

// usize 在 64位操作系统 u64, 32位操作系统 u32
pub fn get_args() -> MyResult<Config> {
    // 成功时返回Some(Config)
    // 失败则返回一个Err(Box<dyn Error>)
    let matches = Command::new("hear")
        .version("0.1.0")
        .author("kyousuke")
        .about("Rust head")
        .arg(
            // 文件名称
            // 用"-"表示默认的值
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s), [defalut \"-\"]")
                .num_args(0..)
                .default_value("-"), // 创建一个叫做files的参数Arg, 值占位符时FILES, --help打印信息在help中
                                     // nums_arg指定可以选择0到多个文件名
                                     // 当不传入值时默认值是"-"意味着从标准输入读取
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines [default 10]")
                .value_parser(RangedU64ValueParser::<usize>::new().range(1..))
                // value_parser尝试将接受的值按照给定的参数进行解析
                // 这里是将其转换为usize，>=1
                // 把字符串解析成 usize；
                // 数值必须大于或等于 1；
                // 解析失败时，由 Clap 输出包含参数名称的标准错误。
                .default_value("10") // 创建一个叫做lines的OPTION变量，短参数名称叫做-n
                .conflicts_with("bytes"), // 当设置时置为true
                                          // 默认值为10, 不适用action就默认接受值
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .value_parser(RangedU64ValueParser::<usize>::new().range(1..))
                .conflicts_with("lines"),
        )
        .get_matches();
    // clap返回的值默认是字符串，需要进行字符串提取
    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        // lines: matches
        //     .get_one::<String>("lines")
        //     .map(|val| parse_positive_int(val))
        //     .transpose()
        //     .map_err(|e| format!("illegal line count -- {}", e))?
        //     .unwrap(),
        lines: *matches.get_one::<usize>("lines").unwrap(),
        // matches.get_one<String>("lines")返回一个Option<&String>（这里必定是Some变体()
        // 应用map方法将Option<&String> 转换为Option<Result<usize, Box<dyn Error>>
        // 目标类型为闭包返回的类型
        // bytes: matches
        //     .get_one::<String>("bytes")
        //     .map(|val| parse_positive_int(val))
        //     .transpose()
        //     .map_err(|e| format!("illegal byte count -- {}", e))?,
        bytes: matches.get_one::<usize>("bytes").copied(),
        // 一个Result<T, E>类型调用map_err, 返回Result<T, F>
        // 如果调用的实例是Ok变体，不执行闭包，返回
        // 如果调用的实例是Err变体，提取出变体中的值，运用在闭包中，并进行返回一个Err<F>变体
        // get_one::<String>返回一个Option(&String)
        // map接受一个Option<&String>
        // 返回Option<Result<usize, Box<dyn Error>>
        // 如果接受的是None, 直接返回None
        // 如果是Option<&String>
        // 将&String提取到闭包中执行
        // 然后将类型转换为
        // Result<Option<usize>, Box<dyn Error>
    })
}
pub fn run(config: Config) -> MyResult<()> {
    // 判断是否存在多个输入文件
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                    // 如果当前确实有多个输入文件，输出一个标题
                    // // 如果要求按照字节读取
                    // if let Some(mut bytes) = config.bytes {
                    //     for byte in file.bytes() {
                    //         // 按照字节读取
                    //         let byte = byte?;
                    //         print!("{}", byte as char);
                    //         bytes -= 1;
                    //         if bytes == 0 {
                    //             break;
                    //         }
                    //     }
                    // } else {
                    //     // 输出指定行数
                    //     let mut lines = config.lines;
                    //     for line in file.lines() {
                    //         let line = line?;
                    //         print!("{}", line);
                    //         lines -= 1;
                    //         if lines == 0 {
                    //            break;
                    //         }
                    //     }
                    // }
                    // take()方法接受一个usize，表示要拿取多少的行
                    //for line in file.lines().take(config.lines) {
                      //  println!("{}", line?);
                    //}
                    // BufRead::lines读取文件时，返回的每一个字符串末尾不会包含换行符
                    // BufRead::read_line方法从底层流读取字节，直到遇到换行分隔符或者文件末尾
                    // 找到分隔符后，分隔符添加到buf中
                    if num_files > 1 {
                        println!(
                        "{} ==> {} <==", if file_num > 0 {"\n"} else {""},
                            filename
                        )
                    }
                    if let Some(num_bytes) = config.bytes {
                        let mut handle = file.take(num_bytes as u64);
                        // take方法表示从file中读取最多num_bytes个字节
                        let mut buffer = vec![0; num_bytes];
                        let bytes_read = handle.read(&mut buffer)?; //从handle中读取到buffer中去
                        print!("{}",
                            String::from_utf8_lossy(&buffer[..bytes_read])
                        );
                        // 将buffer中的字节转换为utf-8的字符串, 如果字符串无效，返回未知字符
                        // let bytes = file.bytes().take(num_bytes).collect::<Vec<_>>();
                        // ::<>类型注解
                    } else {
                        let mut line = String::new();
                        for _ in 0..config.lines {
                            let bytes = file.read_line(&mut line)?;
                            // read_line方法获取一个Result
                            // read_line方法读取的是所有字节
                            // 返回的是一个Result变体，Ok()存储获取了多少字节
                            // 如果是Ok(0)， 说明读取的是EOF
                            if bytes == 0 {
                                break; // 读取了EOF，可以停止了
                            }
                            print!("{line}");
                            line.clear(); // read_line将读取的内容append到line中，所以要清空

                        }
                    }
            }
        }
    }
    Ok(())
}
// // 将字符串切片尝试解析为usize类型
// fn parse_positive_int(val: &str) -> MyResult<usize> {
//     match val.parse() {
//         Ok(n) if n > 0 => Ok(n),
//         _ => Err(From::from(val)),
//         // Err(Into::into(val)) || Err(val.into())
//     }
// }
// // std::convert::From trait 是一个强制类型转换
// // 在执行错误处理时特别有用
// // 也就是该实例中将&str转换为Error类型，std::convert::Into 也可以

// #[test]
// fn test_parse_positive_int() {
//     // 3是合法的usize
//     let res = parse_positive_int("3");
//     assert!(res.is_ok()); // 判断res是否是ok变体
//     assert_eq!(3, res.unwrap());

//     // foo不是合法的usize
//     let res = parse_positive_int("foo");
//     assert!(res.is_err()); // 判断res是否是err变体
//     assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
//     // 将res这个err变体的值提取出来转换为string

//     // 0值也会导致错误
//     let res = parse_positive_int("0");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "0".to_string());
// }
pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), // 创建一个BufReader,其输入从stdin输入
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), // 打开文件，返回文件句柄，然后封装到BufReader
    }
}
