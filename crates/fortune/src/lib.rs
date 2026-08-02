use clap::{Arg, ArgAction, Command};
use rand::prelude::SliceRandom;
use rand::{SeedableRng, rngs::StdRng};
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    source: Vec<String>,    // 输入源文件
    pattern: Option<Regex>, // -m选项下的匹配模式
    seed: Option<u64>,      // 随机搜索种子
}
#[derive(Debug)]
struct Fortune {
    source: String,
    text: String,
} // source是源文件名称，text是某一个文本
// 之所以需要是因为-m选项进行匹配时需要知道所有的行内容和所属源文件
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("fortune")
        .version("0.1.0")
        .author("kyousuke")
        .about("Rust fortune")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input files or directories")
                .required(true)
                .num_args(1..),
            // 创建一个参数file，其value_name是FILE，必须填，且填1..多个
        )
        .arg(
            Arg::new("pattern")
                .short('m')
                .long("pattern")
                .value_name("PATTERN")
                .help("Pattern")
                .num_args(1),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive pattern matching")
                .action(ArgAction::SetTrue)
                .requires("pattern"),
        )
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .value_name("SEED")
                .help("Random seed")
                .value_parser(clap::value_parser!(u64))
                .num_args(1),
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .map(|val| {
            RegexBuilder::new(val)
                .case_insensitive(matches.get_flag("insensitive"))
                .build()
                .map_err(|_| format!("Invalid --pattern \"{}\"", val))
        })
        .transpose()?;
    Ok(Config {
        source: matches
            .get_many::<String>("file")
            .unwrap()
            .cloned()
            .collect(),
        pattern,
        seed: matches.get_one::<u64>("seed").copied(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.source)?;
    let fortunes = read_fortunes(&files)?;

    if let Some(pattern) = config.pattern {
        let mut prev_source = None;
        // fortune是所有匹配的值
        for fortune in fortunes
            .iter()
            .filter(|fortune| pattern.is_match(&fortune.text))
        {
            // 返回Option<&String>
            // 如果是None直接进入
            // 如果不是，判断pre_source如果不和之前打印的一样，打印
            if prev_source.as_ref().map_or(true, |s| s != &fortune.source) {
                eprintln!("({})\n%", fortune.source);
                prev_source = Some(fortune.source.clone());
            }
            println!("{}\n%", fortune.text);
        }
    } else {
        println!(
            "{}",
            pick_fortune(&fortunes, config.seed)
                .or_else(|| Some("No fortunes found".to_string()))
                .unwrap()
        );
    }
    Ok(())
}
// fn parse_u64(val: &str) -> MyResult<u64> {
//     val.parse::<u64>().map_err(|_e| format!("\"{}\" not a valid integer", val).into())
//     // String.into()得到一个错误类型
// }

// find_files应该返回排序后的路径
fn find_files(paths: &[String]) -> MyResult<Vec<PathBuf>> {
    // 用于表示路径的结构体
    // 1. Path，支持许多用于检查路径的操作，包括分割，判断是否为绝对路径，提取文件名等
    // 但是该类型是一个不定长类型，所以我们返回时需要返回他的引用
    // 但是由于所有权系统，以及生命周期，我们返回一个悬垂引用，拒绝
    // 2. PathBuf, 也可以表示路径，不过他拥有所有权，可修改。二者关系类似于
    // PathBuf -> Path
    // String -> &str
    // OsStr类型时Rust中表示操作系统首先字符串表示法的类型。OsStr是借用的，所有权版本是OsString

    // let mut res = vec![];
    // for path in paths {
    //     let mut path_buf = PathBuf::new();
    //     for entry in WalkDir::new(path) {
    //         let entry = entry?;
    //         // 如果entry的类型是文件，且结尾不是 ".dat"
    //         // 加入，否则忽略
    //         if entry.file_type().is_file() &&
    //             !entry.file_name().display().to_string()
    //                 .ends_with(".dat") {
    //             path_buf.push(entry.file_name().display().to_string());
    //         }
    //     }
    //     res.push(path_buf);
    // }
    // res.sort();
    // Ok(res)

    let dat = OsStr::new("dat"); // 操作系统首选字符串
    let mut files = vec![];

    for path in paths {
        // 获取所有输入的元数据
        // 如果失效，直接返回
        // 如果成功，对files进行扩充
        // 创建一个WalkDir，变为迭代器
        // 对筛选出所有变体是ok的元素
        // 进一步进行筛选
        // 筛选出是文件类型并且扩展名不是dat的值
        // 将entry变换为PathBuf
        match fs::metadata(path) {
            Err(e) => return Err(format!("{}: {}", path, e).into()),
            Ok(_) => files.extend(
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| e.file_type().is_file() && e.path().extension() != Some(dat))
                    .map(|e| e.path().into()),
            ),
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn read_fortunes(paths: &[PathBuf]) -> MyResult<Vec<Fortune>> {
    // 读取所有的Fortune
    let mut fortunes = vec![];
    let mut buffer = vec![];
    for path in paths {
        // 遍历每一个可访问的路径
        let basename = path.file_name().unwrap().to_string_lossy().into_owned();
        // basename是当前路径的文件名称
        let file = File::open(path)
            .map_err(|e| format!("{}: {}", path.to_string_lossy().into_owned(), e))?;
        // file是文件句柄
        // 利用filter_map获取所有有效行
        for line in BufReader::new(file).lines().filter_map(Result::ok) {
            if line == "%" {
                if !buffer.is_empty() {
                    fortunes.push(Fortune {
                        source: basename.clone(),
                        text: buffer.join("\n"),
                    });
                    buffer.clear();
                }
            } else {
                buffer.push(line); // 普通行进 buffer，挂在这里
            }
        }
        // 文件末尾 flush 残留的最后一条
        if !buffer.is_empty() {
            fortunes.push(Fortune {
                source: basename.clone(),
                text: buffer.join("\n"),
            });
            buffer.clear();
        }
    }
    Ok(fortunes)
}

// 如果没有种子，通过使用rand::thread_rng创建一个由系统提供种子的RNG
// 如果由种子，使用rand::rngs::StdRng::seed_from_u64
// 最后使用SliceRandom::choose来进行随机选择
fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    if let Some(val) = seed {
        let mut rng = StdRng::seed_from_u64(val);
        fortunes.choose(&mut rng).map(|f| f.text.to_string())
    } else {
        let mut rng = rand::thread_rng();
        fortunes.choose(&mut rng).map(|f| f.text.to_string())
    }
}
#[cfg(test)]
mod tests {
    use super::{Fortune, find_files, pick_fortune, read_fortunes};
    use std::path::PathBuf;
    // #[test]
    // fn test_parse_u64() {
    //     let res = parse_u64("a");
    //     assert!(res.is_err());
    //     assert_eq!(res.unwrap_err().to_string(), "\"a\" not a valid integer");
    //
    //     let res = parse_u64("0");
    //     assert!(res.is_ok());
    //     assert_eq!(res.unwrap(), 0);
    //
    //     let res = parse_u64("4");
    //     assert!(res.is_ok());
    //     assert_eq!(res.unwrap(), 4);
    // }

    #[test]
    fn test_find_files() {
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );

        // 找不到错误文件
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // 查看所有输入我呢见，排除".dat"后缀
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());
        // 检查文件数目与顺序
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);

        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }

    #[test]
    fn test_read_fortunes() {
        // Parses all the fortunes without a filter
        let res = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 6);
            assert_eq!(
                fortunes.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
                A. Collared greens."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\n\
                A: A bad idea (bad-eye deer)."
            );
        }

        // Filters for matching text
        let res = read_fortunes(&[
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 11);
    }

    #[test]
    fn test_pick_fortune() {
        // Create a slice of fortunes
        let fortunes = &[
            Fortune {
                source: "fortunes".to_string(),
                text: "You cannot achieve the impossible without \
                      attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Assumption is the mother of all screw-ups.".to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];

        // Pick a fortune with a seed
        assert_eq!(
            pick_fortune(fortunes, Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
