use clap::{Command, Arg, ArgAction};
use std::error::Error;
use regex::{Regex, RegexBuilder}; // 正则表达式部分
use walkdir::WalkDir;
use std::fs:: File;
use std::io::{self, BufRead, BufReader};
use std::mem;
// 自定义错误类型
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex, // 正则表达式 程序的不区分大小写选项用regex::RegexBuilder通过case_insensitive方法创建
    invert_match: bool, // 匹配反转
    count: bool, // 是否计数
    recursive: bool, // 递归遍历目录
    files: Vec<String>, // 输入文件，默认-
}

pub fn get_args() -> MyResult<Config> {

    let matches = Command::new("grepr")
        .author("kyousuke")
        .version("0.1.0")
        .about("Rust grep")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .required(true)
                .help("Search pattern")
                .num_args(1)
                // pattern是必须接受的
            // 没有short，long的可选参数叫做位置参数
            // 位置参数在clap声明中的顺序会依次赋值一个index
            // 从1开始
            // 其中必选参数的index必须比可选的位置参数小
            // 否则无法解析出必选参数的值
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .num_args(0..)
                .help("Input file(s)")
                .default_value("-")
            // 输入文件的参数设置
        )
        .arg(
            Arg::new("count")
                .value_name("COUNT")
                .short('c')
                .long("count")
                .help("Count occurrences")
                .action(ArgAction::SetTrue)
            // -c不与任何冲突
        )
        .arg(
            Arg::new("insensitive")
                .value_name("INSENSITIVE")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("invert_match")
                .value_name("INVERT-MATCH")
                .short('v')
                .long("invert-match")
                .help("Invert match")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("recursive")
                .value_name("RECURSIVE")
                .help("Recursive search")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    Ok(Config {
        files: matches
            .get_many::<String>("file")
            .unwrap()
            .cloned()
            .collect(),
        count: matches.get_flag("count"),
        invert_match: matches.get_flag("invert_match"),
        recursive: matches.get_flag("recursive"),
        pattern: matches
            .get_one::<String>("pattern")
            .map(|pattern|
                RegexBuilder::build( // 编译正则表达式
                RegexBuilder::new(&pattern) // 创建正则表达式
                .case_insensitive(matches.get_flag("insensitive"))
                ).map_err(|_err| format!("Invalid pattern \"{}\"", pattern)))
            .transpose()?
            .unwrap(),
        // get_one获取模式输入得到Option<&String>，并且必定是Some(val)变体
        // 应用map闭包，将Option(&String) -> Option<Result<Regex, regex Error>>
        // map闭包的逻辑为，绑定获取的&String pattern
        // 利用该pattern创建一个regex得到一个result类型
        // 应用map_err，如果创建成功，返回Ok(result)变体
        // 如果失败，返回一个Err()变体
        // 然后通过map逃入一个option中
        // 通过transpose和?将错误上传，然后解包
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // 打印正则表达式使用Regex::as_str方法，接受一个&regex
    let entries = find_files(&config.files, config.recursive); // 获取所有输入的文件目录
    let num_files = entries.len(); // 有多少个输入数目
    let print = |filename: &str, val: &str| {
        if num_files > 1 {
            print!("{}:{}", filename, val);
        } else {
            print!("{}", val);
        }
    } ;
    for entry in entries {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(filename) => match open(&filename) {
                Err(e) => eprintln!("{}: {}", filename, e), // 打开文件失败
                Ok(file) => {
                    match find_lines(file, &config.pattern, config.invert_match) {
                        Err(e) => eprintln!("{}", e),
                        Ok(matches) => {
                            if config.count {
                                print(&filename, &format!("{}\n", matches.len()));
                            } else {
                                for line in &matches {
                                    print(&filename, line);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

// paths 接受可能包含文件或者目录名称的字符传向量
// 返回结果为包含有效文件的名称字符串或者错误信息
fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {
    let mut result: Vec<MyResult<String>> = vec![];
    // 遍历每一个paths，创建一个entry
    // 然后递归使用每一个entry
    // 获取每一个目录或者文件
    // 判断类型，如果当前时文件，加入到result
    // 如果当前时目录，但是recursive是true，加入result
    for path in paths {
        if path == "-" {
            result.push(Ok(path.clone()))
        } else {
            for entry in WalkDir::new(path) {
                match entry {
                    Err(e) => result.push(Err(Box::new(e))), // 如果访问条目失败，加入错误信息
                    Ok(entry) if entry.file_type().is_file() =>
                        result.push(Ok(entry.path().display().to_string())),
                    Ok(entry) if entry.file_type().is_dir() && !recursive => {
                        result.push(Err(From::from(format!(
                            "{} is a directory",
                            entry.path().display()
                        ))));
                        break;
                    }
                    Ok(_) => {},
                }
                // WalkDir::new()创建迭代器。直到将所有子目录遍历完毕
            }
        }
    }
    result
}

fn find_lines<T: BufRead>(mut file: T, pattern: &Regex, invert_match: bool) -> MyResult<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?; // 将每一行包括换行符读取到line中
        if bytes == 0 {
            break;
        }
        // 1. 如果匹配且不反转，加入
        // 2. 如果不匹配且反转加入
        // 3. 如果匹配且反转，不加入
        // 4. 如果不匹配且不反转，不加入
        if pattern.is_match(&line) ^ invert_match {
            matches.push(mem::take(&mut line));
            // mem::take可以直接夺取所有权
        }
        line.clear();
    }
    Ok(matches)
}
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
         _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
#[cfg(test)]
mod tests {
    use super::{find_files, find_lines};
    use pretty_assertions::assert_eq;
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When inverted, the function should match the other two lines
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files =
            find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}

