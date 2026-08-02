use ansi_term::Style; // 用于高亮控制台当天日期的值
use chrono::NaiveDate; // NaiveDate是一个表示不带时区的ISO 8601日历日期.
use chrono::{Datelike, Local};
use clap::{Arg, ArgAction, Command};
use itertools::izip;
use std::error::Error;
use std::str::FromStr;

// 合并行
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const LINE_WIDTH: usize = 22; // 每一行有22格
#[derive(Debug)]
pub struct Config {
    month: Option<u32>, // 输入的位置参数1，用于记录指定的月份，并且必须在使用年份的情况下指定
    year: i32,          // 月份是必要的，但是不是必须输入的
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("calr")
        .version("0.1.0")
        .author("kyousuke")
        .about("Rust cal")
        .arg(
            Arg::new("year")
                .value_name("YEAR")
                .help("Year (1-9999)")
                .num_args(1)
                .value_parser(clap::value_parser!(i32).range(1..=9999)),
            // 创建一个位置参数year,值占位符是YEAR， 只可接受一个，如果接受，范围为1，9999
        )
        .arg(
            Arg::new("show_year")
                .value_name("SHOW_YEAR")
                .short('y')
                .long("year")
                .help("Show whole current year")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["year", "month"]),
        )
        .arg(
            Arg::new("month")
                .short('m')
                .value_name("MONTH")
                .help("Month name or number (1-12)")
                .num_args(1),
        )
        .get_matches();
    // 当不带参数运行时，使用默认的年份和月份
    // let today = Local::now(); // Local::now()可以获得表示当前本地时区的年份和月份
    // let year = matches.get_one::<i32>("year")
    //     .copied();
    // let month = matches.get_one::<u32>("month")
    //     .copied();
    // let show_year = matches.get_flag("show_year");
    // let (year, month) = match (year, month) {
    //     // 1. 如果year和month都有输入
    //     (Some(year), Some(_)) => (year, month),
    //     // 2. 如果year有输入，month没有输入
    //     (Some(year), None) => (year, None),
    //     // 3. 如果都没有输入且当前没有—y
    //     (None, None) if !show_year => (today.year(), Some(today.month())),
    //     (None, None) if show_year => (today.year(), None),
    //     // 如果year没输入，month有输入
    //     (None, Some(_)) => (today.year(), month),
    //     _ => unreachable!(),
    // };
    let mut month = matches
        .get_one::<String>("month")
        .map(|month| parse_month(month))
        .transpose()?;
    let mut year = matches.get_one::<i32>("year").copied();
    let today = Local::now();
    if matches.get_flag("show_year") {
        // 如果有-y选项
        // 说明没有任何输入
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }
    Ok(Config {
        month,                                      // today.month()获取当前的月份
        year: year.unwrap_or_else(|| today.year()), // 获取当前的年份
        today: today.date_naive(),                  // 获取今天的日期
    })
}
// 泛型函数parse_int
fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    val.parse::<T>()
        .map_err(|_| format!("Invalid integer \"{}\"", val).into())
}

// fn parse_year(year: &str) -> MyResult<i32> {
//     let year = parse_int::<i32>(year);
//     match year {
//         Ok(year) if year >= 1 && year <= 9999 => Ok(year),
//         Ok(year) => Err(From::from(format!("year \"{}\" not in the range 1 through 9999", year))),
//         Err(e) => Err(e),
//     }
// }

fn parse_month(month: &str) -> MyResult<u32> {
    match parse_int::<u32>(month) {
        Ok(num) if num >= 1 && num <= 12 => Ok(num),
        Ok(_num) => Err(From::from(format!(
            "month \"{}\" not in the range 1 through 12",
            month
        ))),
        _ => {
            // 将输入的month小写话
            let lower = &month.to_lowercase();
            let matches: Vec<_> = MONTH_NAMES
                .iter() // 获取迭代器
                .enumerate() // 获取数对
                .filter_map(|(i, name)| {
                    // 获取前缀成功匹配lower的月份对应i32值
                    if name.to_lowercase().starts_with(lower) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect();
            if matches.len() == 1 {
                Ok(matches[0] as u32)
            } else {
                Err(format!("Invalid month \"{}\"", month).into())
            }
        }
    }
}

// 输出某一个月的输出
fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap(); // 当前年月的第一天
    let mut days: Vec<_> = (1..first.weekday().number_from_sunday())
        .map(|_| "  ".to_string()) // ← 只有 1 个空格
        .collect();
    // weekday()是获取当前月第一天是星期几，然后number_from_Sunday()求出差几天
    // 首先插入对应的空格
    let is_today = |day: u32| year == today.year() && month == today.month() && day == today.day(); // 判断某一天是否是今天
    let last = last_day_in_month(year, month); // 当前年月的最后一天
    days.extend((first.day()..=last.day()).into_iter().map(|num| {
        let fmt = format!("{:>2}", num);
        if is_today(num) {
            Style::new().reverse().paint(fmt).to_string()
        } else {
            fmt
        }
    }));
    // 从第一天到最后一天进行扩充，将其按照2个空格右对齐
    // 如果是今天，style一下
    let month_name = MONTH_NAMES[month as usize - 1];
    let mut lines = Vec::with_capacity(8);
    lines.push(format!(
        "{:^20}  ",
        if print_year {
            format!("{} {}", month_name, year)
        } else {
            month_name.to_string()
        }
    )); // 首先将第一行加入到结果中

    lines.push("Su Mo Tu We Th Fr Sa  ".to_string());
    for week in days.chunks(7) {
        lines.push(format!(
            "{:width$}  ",
            week.join(" "),
            width = LINE_WIDTH - 2
        ));
        // 将days按照大小为7的尺寸拆分为若干个chunks
        // 每一个chunks合并
    }
    while lines.len() < 8 {
        lines.push(" ".repeat(LINE_WIDTH));
    }
    lines
}

// 返回任意月份的最后一天的NaiveDate
fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    }; // 计算出下一个月的第一个天，然后找到其前驱
    NaiveDate::from_ymd_opt(y, m, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
}
pub fn run(config: Config) -> MyResult<()> {
    match config.month {
        Some(month) => {
            let lines = format_month(config.year, month, true, config.today);
            println!("{}", lines.join("\n"));
        }
        None => {
            println!("{:>32}", config.year);
            let months: Vec<_> = (1..=12)
                .into_iter()
                .map(|month| format_month(config.year, month, false, config.today))
                .collect();

            for (i, chunk) in months.chunks(3).enumerate() {
                if let [m1, m2, m3] = chunk {
                    for lines in izip!(m1, m2, m3) {
                        println!("{}{}{}", lines.0, lines.1, lines.2);
                    }
                    if i < 3 {
                        println!();
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{format_month, last_day_in_month, parse_int, parse_month};
    use chrono::NaiveDate;

    #[test]
    fn test_parse_int() {
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    // #[test]
    // fn test_parse_year() {
    //     let res = parse_year("1");
    //     assert!(res.is_ok());
    //     assert_eq!(res.unwrap(), 1i32);
    //
    //     let res = parse_year("9999");
    //     assert!(res.is_ok());
    //     assert_eq!(res.unwrap(), 9999i32);
    //
    //     let res = parse_year("0");
    //     assert!(res.is_err());
    //     assert_eq!(
    //         res.unwrap_err().to_string(),
    //         "year \"0\" not in the range 1 through 9999"
    //     );
    //
    //     let res = parse_year("10000");
    //     assert!(res.is_err());
    //     assert_eq!(
    //         res.unwrap_err().to_string(),
    //         "year \"10000\" not in the range 1 through 9999"
    //     );
    //
    //     let res = parse_year("foo");
    //     assert!(res.is_err())
    // }

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1 through 12"
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1 through 12"
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
        let leap_february = vec![
            "   February 2020      ",
            "Su Mo Tu We Th Fr Sa  ",
            "                   1  ",
            " 2  3  4  5  6  7  8  ",
            " 9 10 11 12 13 14 15  ",
            "16 17 18 19 20 21 22  ",
            "23 24 25 26 27 28 29  ",
            "                      ",
        ];
        assert_eq!(format_month(2020, 2, true, today), leap_february);

        let may = vec![
            "        May           ",
            "Su Mo Tu We Th Fr Sa  ",
            "                1  2  ",
            " 3  4  5  6  7  8  9  ",
            "10 11 12 13 14 15 16  ",
            "17 18 19 20 21 22 23  ",
            "24 25 26 27 28 29 30  ",
            "31                    ",
        ];
        assert_eq!(format_month(2020, 5, false, today), may);

        let april_hl = vec![
            "     April 2021       ",
            "Su Mo Tu We Th Fr Sa  ",
            "             1  2  3  ",
            " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29 30     ",
            "                      ",
        ];
        let today = NaiveDate::from_ymd_opt(2021, 4, 7).unwrap();
        assert_eq!(format_month(2021, 4, true, today), april_hl);
    }
    #[test]
    fn test_last_day_in_month() {
        assert_eq!(
            last_day_in_month(2020, 1),
            NaiveDate::from_ymd_opt(2020, 1, 31).unwrap()
        );
        assert_eq!(
            last_day_in_month(2020, 2),
            NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
        );
        assert_eq!(
            last_day_in_month(2020, 4),
            NaiveDate::from_ymd_opt(2020, 4, 30).unwrap()
        );
    }
}
