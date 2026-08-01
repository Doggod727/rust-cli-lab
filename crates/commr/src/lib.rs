use crate::Column::*;
use clap::{Arg, ArgAction, Command};
use std::cmp::Ordering::*;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
type MyResult<T> = Result<T, Box<dyn Error>>;

// Column枚举用来表示输出应该在哪一个列
enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}
#[derive(Debug)]
pub struct Config {
    file1: String,     // 输入文件1
    file2: String,     // 输入文件2
    show_col1: bool,   // 是否显示输出的第一列
    show_col2: bool,   // 是否显示输出的第二列
    show_col3: bool,   // 是否显示输出的第三列
    insensitive: bool, // 是否忽略大小写比较
    delimiter: String, // 输出的列分割符，默认为制表符
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("commr")
        .version("0.1.0")
        .author("kyousuke")
        .about("Rust comm")
        .arg(
            Arg::new("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .required(true)
                .num_args(1),
            // 创建位置参数1， file1, 必须填，且只能填1个
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true)
                .num_args(1),
            // 创建位置参数2， file2, 必须填， 且只能填一个
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .help("Case-insensitive comparison of lines")
                .action(ArgAction::SetTrue),
            // 创建一个选项-i, 当出现就置为true
        )
        .arg(
            Arg::new("show_col1")
                .short('1')
                .help("Suppress printing of column 1")
                .action(ArgAction::SetFalse),
            // 创建选项-1，当出现时置为false
        )
        .arg(
            Arg::new("show_col2")
                .short('2')
                .help("Suppress printing of column2")
                .action(ArgAction::SetFalse),
            // 创建选项-2
        )
        .arg(
            Arg::new("show_col3")
                .short('3')
                .help("Suppress printing of column 3")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new("delimiter")
                .help("Output delimiter")
                .short('d')
                .long("output-delimiter")
                .num_args(1)
                .default_value("\t"), // 创建一个参数delimiter，参数需要取值，且需要取一个值，默认值为制表符
        )
        .get_matches();

    let file1 = matches.get_one::<String>("file1").unwrap().to_string();
    let file2 = matches.get_one::<String>("file2").unwrap().to_string();
    Ok(Config {
        file1,
        file2,
        show_col1: matches.get_flag("show_col1"),
        show_col2: matches.get_flag("show_col2"),
        show_col3: matches.get_flag("show_col3"),
        insensitive: matches.get_flag("insensitive"),
        delimiter: matches.get_one::<String>("delimiter").unwrap().to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from(String::from(
            "Both input files cannot be STDIN (\"-\")",
        )));
    }

    // 创建闭包case
    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let print = |col: Column| {
        let mut columns = vec![]; // 每一行的具体打印内容
        match col {
            // 如果当前就要打印到第一列
            Col1(val) => {
                // config要显示第一列
                if config.show_col1 {
                    columns.push(val);
                }
            }
            // 打印第二列
            Col2(val) => {
                // 要显示第二列
                if config.show_col2 {
                    if config.show_col1 {
                        columns.push("")
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if config.show_col3 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    if config.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        }
        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };
    let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
    // 打开文件，如果成功打开，获取lines迭代器
    // 应用filter_map迭代器适配器，应用闭包Result::ok
    // 对于迭代器lines返回的每一个元素Result
    // 如果是Ok变体，获取值，返回Some(val)
    // 如果是Err变体，返回None变体
    // 应用filter_map，将返回val的获取
    let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    // 如果还有任何一个文件没有读完，就继续
    // 如果两个都没有读完，就说明都是Some变体，都next
    // 如果file1读完了，file2没读完，第一个是None变体，后续业不会变，line2是Some变体
    // 如果lines2读完了，line2一直是None变体，lines1没读完，line1是Some变体
    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            // 如果读取到了两个行
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Equal => {
                    // 如果两个相同，打印到第三行
                    // 并且分别读取下一行
                    print(Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    // 如果val1更小，先打印val1
                    print(Col1(val1));
                    line1 = lines1.next();
                }
                Greater => {
                    // 如果val2更小，先打印val2
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val), None) => {
                print(Col1(val)); // 只有一个行，直接打印
                line1 = lines1.next();
            }
            (None, Some(val)) => {
                print(Col2(val)); // 只有一个，直接打印
                line2 = lines2.next();
            }
            _ => (),
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(&filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}
