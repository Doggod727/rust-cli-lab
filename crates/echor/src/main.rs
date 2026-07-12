use clap::{Command, Arg, ArgAction}; // clap::Command对应于2.x的clap::App
// 其是一个结构体
// 用来解析命令行参数
fn main() {
    // println!("{:?}", env::args());
    // note: 没有显示返回类型的函数和表达式返回() 单元类型，类似空值但是不是
    // cargo run -p echor -- n [args]
    // 用--隔离cargo run
    // echo的其他所有参数都是位置参数
    // -n是一个可选参数，是可以忽略的
    // 可选参数-c | --word两种形式
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Kyousuke <liiiii05262024@outlook.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print new line")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    // 创建了一个叫做echor的命令行程序
    // .version()标注了当前命令行程序的版本
    // .author()标记作者信息
    // .about()标注简短的描述
    // .get_matches() 按照定义的规则取解析命令行程序
    // cargo run -h 可以查看了
    // clap自己定义了-h可选参数的行为
    // arg()接受Arg结构体，用来描述当前命令行程序接受参数的方式
    // Arg结构体描述一个参数长什么样子，有什么规则
    // Arg::new("text")创建一个参数，其名字叫做text
    // .value_name("TEXT")表示通过——help打印时的当前参数的占位符名称
    // .help()说明文档的说明文字
    // .required(true) 当前参数必须传入
    // .num_args()至少有多少个参数
    // .short('n')段参数-n
    // .action(ArgAction::SetTrue) 开关，出现了-n就是true
    // println!("{:#?}", matches);
    // 通过Command获取了参数之后，我们要提取出来
    let text : Vec<String> = matches.get_many::<String>("text")
        .unwrap()
        .cloned()
        .collect();
    // get_many获取某一个参数的值的迭代器，cloned通过克隆获取所有权，还是迭代器
    // collect收集起来
    let omit_newline = matches.get_flag("omit_newline"); // get_flag判断可选参数是否存在
    // Vec::join函数可以针对字符串向量，将其合并为由空格分离的字符串
    let ending = if omit_newline { "" } else { "\n" };
    // 没有else分支的if语句是返回单元类型
    print!("{}{}", text.join(" "), ending);
}
