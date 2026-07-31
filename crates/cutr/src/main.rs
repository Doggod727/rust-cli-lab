// use csv::{ReaderBuilder, StringRecord};
// use std::fs::File;
fn main() {
    // get_args()方法获取CLI读取的参数
    // 返回MyResult<Config>结构体(Result<Config, Box<dyn Error>>
    // 如果读取成功，返回Ok(Config)变体，使用and_then方法，将Config提取出来并且运行在
    // 闭包cutr::run上，该方法也返回MyResult<()>结构体
    // 如果run成功，返回Ok(())变体
    // 此时条件模式匹配结束，正常返回
    // 如果get_args()失败，返回Err(Box<dyn Error>)
    // 不调用and_then()，进行模式匹配
    // 打印错误
    // 如果get_args()成功，run失败，返回Err变体
    // 模式匹配，打印错误

    if let Err(e) = cutr::get_args().and_then(cutr::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    // 创建一个csv解析句柄，分隔符初始化为'b', 从books.csv文件中读取
    // from_reader()接受一个实现了Read Trait的值
    // let mut reader = ReaderBuilder::new()
    //     .delimiter(b',')
    //     .from_reader(File::open("./crates/cutr/tests/inputs/books.csv")?);
    // // reader.headers()方法返回的是第一行的值作为列名称，返回类型为StringRecord
    // println!("{}", fmt(reader.headers()?));
    // // records是一个迭代器
    // for record in reader.records() {
    //     println!("{}", fmt(&record?));
    // }
    // Ok(())
}
// fn fmt(rec: &StringRecord) -> String {
//     rec.into_iter().map(|v| format!("{:20}", v)).collect()
// }
