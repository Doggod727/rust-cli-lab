use crate::EntryType::*;
use clap::{Arg, ArgAction, Command};
use regex::Regex; // 正则表达式
use std::error::Error;
use walkdir::{DirEntry, WalkDir}; // 让我们可以直接使用Dir变体
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}
#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("finder")
        .author("kyousuke")
        .version("0.1.0")
        .about("Rust find")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .num_args(0..),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .help("Name")
                .long("name")
                .short('n')
                .action(ArgAction::Append)
                .num_args(0..),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .help("Entry type")
                .long("type")
                .short('t')
                .value_parser(["f", "d", "l"]) // 限定取值范围
                .action(ArgAction::Append) // 允许一个选项出现多次，其值append到对应的列表中
                .num_args(0..), // 每一次选项可以拿多少值
        )
        .get_matches();
    let names = matches
        .get_many::<String>("names")
        .map(|vals| {
            vals.map(|name| {
                Regex::new(&name).map_err(|_err| format!("Invalid --name \"{}\"", name))
            })
            .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();
    let entry_types = matches
        .get_many::<String>("types")
        .map(|vals| {
            vals.map(|val| match val.as_str() {
                "d" => Dir,
                "f" => File,
                "l" => Link,
                _ => unreachable!("Invalid entry type"), // 绝对不可到达
            })
            .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths: matches
            .get_many::<String>("path")
            .unwrap()
            .cloned()
            .collect(),
        names,
        entry_types,
    })
}

#[test]
fn test_regex() {
    let re = Regex::new(".*[.]csv$").unwrap(); // 我们可以在模式末尾添加一个$来表示字符串的结尾
    assert!(re.is_match("foo.csv"));
    assert!(!re.is_match(".csv.foo"));
    // 我们也要使用^来表示字符串的开头
}
pub fn run(config: Config) -> MyResult<()> {
    // for path in config.paths {
    //     for entry in WalkDir::new(&path) {
    //         match entry {
    //             Ok(entry) => println!("{:?}", entry.path().display()),
    //             Err(err) => eprintln!("Error: {}", err),
    //         }
    //     }
    // }
    // for path in config.paths {
    //     for entry in WalkDir::new(path) {
    //         match entry {
    //             Err(e) => eprintln!("{}", e),
    //             Ok(entry) => {
    //                 if config.entry_types.is_empty()
    //                     || config.entry_types.iter().any(|entry_type| {
    //                     match entry_type {
    //                         Link => entry.file_type().is_symlink(),
    //                         File => entry.file_type().is_file(),
    //                         Dir => entry.file_type().is_dir(),
    //                     }
    //                 })
    //                 && (config.names.is_empty() ||
    //                  config.names.iter().any(|re|
    //                     re.is_match(&entry.file_name().to_string_lossy())
    //                  )) // config.names.iter()创建了一个迭代器，any迭代器方法判断谓词为，当前的目录名称如果与任意一个names正则匹配，返回true
    //                 {
    //                     println!("{}", entry.path().display());
    //                 }
    //             }
    //         }
    //     }
    // }
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file(),
                    Link => entry.file_type().is_symlink(),
                })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}
