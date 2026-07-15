use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
// 自定义一个错误类型，成功返回(), 错误返回一个trait 对象
type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,          // 用来记录文件名
    number_lines: bool,          // -n参数，表示是否要打印行号
    number_nonblank_lines: bool, // -b
}

// get_args()方法，用来读取参数，并且返回一个Result<Config>
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("kyousuke")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s) [defaults: -] ")
                .num_args(0..)
                .default_value("-"), // 创建一个参数files，其输出名称叫做FILES
                                     // 该参数可以1个或者很多个，也可以不填
                                     // 所以不是一定需要
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines from 1")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number nonblank lines from 1")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_lines"),
        )
        .get_matches();
    // let files = matches.get_many::<String>("files").unwrap_or(Vec::<String>::new())
    //     .cloned()
    //     .collect();// get_many::<String>获取迭代器，元素类型是String
    // 由于可能没有参数
    // let files: Vec<String> = matches
    //     .get_many::<String>("files") // Option<Iter<&String>>
    //     // 1. map自动match，提取Some里的迭代器→vals
    //     // 2. 迭代器转Vec<String>，包回Some → Option<Vec<String>>
    //     .map(|vals| vals.cloned().collect())
    //     // 3. 处理Option外壳：Some取内部vec，None返回空vec
    //     .unwrap_or(vec!["-".to_string()]);
    // let number_lines = matches.get_flag("number_lines");
    // let number_nonblank_lines = matches.get_flag("number_nonblank_lines");
    // 判断number_lines 和 number_nonblank_lines不能同时为真
    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })
}

// 创建实现了BufRead Trait的句柄
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    } // std::io::stdin实现了BufRead trait
    // File::open 返回一个Result
    // 成功就是一个句柄，失败就是一个错误
    // BufRead移除所有换行符
}
// | 运算符，可以将前一个命令的标准输出输入到第二个命令的标准输入
// 当使用-作为文件名时，相同
// < 从指定文件名读入输入并提供给标准输入
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(reader) => {
                // 实现读取功能
                // BufRead::lines()方法获取一个迭代器
                // 迭代器引用的元素是读取文本的每一行的Result
                // for_each是一个消费适配器
                // number功能如何实现呢？
                // let lines = reader.lines();// lines是一个迭代器
                // // 遍历
                // // 当从文件读行时，不会直接从文件句柄获取行，而是获得一个std::io::Result
                // // 由于依赖于外部环境，可能失败
                // for line in lines {
                //     // 如果当前行是空行并且number_nonblank_lines，忽略
                //     let line = line?;
                //     // 如果是-b
                //     if config.number_nonblank_lines {
                //         // 如果是空行
                //         if line.is_empty() {
                //             println!();
                //         } else {
                //             // >表示右对齐
                //             // <左对齐
                //             // ^中间对齐
                //             println!("{count:>6}\t{line}");
                //             count += 1;
                //         }
                //     } else if config.number_lines {
                //         println!("{count:>6}\t{line}");
                //         count += 1;
                //     } else {
                //         println!("{line}");
                //     }
                // }
                // Iterator方式解决
                let mut last_num = 0;
                for (line_num, line) in reader.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{line}", line_num + 1);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            last_num += 1;
                            println!("{:>6}\t{line}", last_num);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
