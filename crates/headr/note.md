# Head Aches
## How head Works head
``head`` 用来打印一个或多个文件的若干行或者若干字节。
``-n``选项可以控制显示的行数，例如
``head -n 2 ./tests/inputs/twelve.txt`` 表示只打印该文本的前两行的值。 如果-n后跟上负数，表示除开后k行全部打印
``head -c 2 ./tests/inputs/twelve.txt`` 表示值打印当前文本的前两个字节。 如果-c后更上负数，表示除开后c的字节全部打印。
当存在多个输入文件时，``head``会在每一个输出内容前加上一行
``==> filename <==``
如果没有文件参数，从标准输入读取输入
如果文件不存在，或者无法访问，输出错误信息后退出.
## Getting Started
### Writing a Unit Test to Parse a String into a Number
所有命令行参数都是字符串！
str::parse()函数可以将字符串切片解析为其他类型
当无法将值解析为目标数字是，返回一个含有Err变体的Result
或者返回一个含有转换后数字的Ok
### Converting Strings into Errors
