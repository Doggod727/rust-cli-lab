### Echo
echo的作用就是接受若干个命令行参数，然后在标准输出流中输出这些输出
echo 会自动将任意数量的空格作为参数分隔符
因此含有空格的参数需要使用引号包括起来
例如
```
echo "Rust has assumed control"
Rust has assumed control
```
如果不加入引号
这四个单词视为四个独立的参数
```angular2html
echo Rust    has assumed   control
Rust has assumed control
```
输出时仅用单个空格隔离各个参数
``echo``本身不能使用 -h 参数来获取帮助文档，而是``man echo``
``echo`` 默认以换行符结束，除非显示指定 ``echo -n [args..]``
``echo [args] > target`` 可以重定向
``diff``工具可以指定两个文件的差异
### 访问命令行参数
```
pub fn args() -> Args
```
``Args``是一个结构体，没有实现Display Trait

```
Rust echo

Usage: echor.exe [OPTIONS] <TEXT>...

Arguments:
  <TEXT>...  Input text

Options:
  -n             Do not print new line
  -h, --help     Print help
  -V, --version  Print version
```
``[]``表示不是必须传入
``<>``表示必须接受
``...``可以接受多个

良好的程序应该将常规输出打印到STDOUT
错误信息输出到STDERR
