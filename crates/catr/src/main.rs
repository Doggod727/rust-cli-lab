fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    // and_then是一个链式链接函数，作用于Result<T, E>
    // 接受一个闭包，该闭包返回Result<U, E>
    // 当前实例为：
    // get_args()返回一个Result<Config, Box<dyn Error>>
    // 调用and_then(catr::run)
    // 1. 如果get_args返回Config
    // 将Config传入到catr::run 返回
    // 2. 如果是Error
    // 直接返回
}
