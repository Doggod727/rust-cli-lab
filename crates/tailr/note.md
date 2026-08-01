# How tail Works tail
`tail` 输出文件的末尾部分，默认输出后10行内容到标准输出。如果有多个输入文件。则每一个输出前会有一个文件名输出。
如果没有输入文件或者输入是'-'，则从标准输入进行读取.
`-c|--bytes=K` 输出最后K个字节； 如果`-c +K`从每一个文件的第K个字节后开始输出.
`-n|--lines=K` 输出最后K行，使用`-n +K` 从每一个文件的第K行开始输出.
`-q` 标注会抑制头文件的输出。

```angular2html
tail tests/inputs/ten.txt
```
默认输出后10行
```angular2html
tail -n 4 tests/inputs/twelve.txt
```
只输出后4行
```angular2html
tail -c 8 tests/inputs/twelve.txt | cat -e
```
只输出后8个字节的内容
```angular2html
tail -n 1 tests/inputs/one.txt blargh tests/inputs/three.txt
```
如果有多个输入文件，每一个输出前有一个文件名称提示
请求比文件本省包含跟多行或者字节不会报错，而是打印整个文件。

```angular2html
tail -n + tests/inputs/twelve.txt
```
从第8行开始打印

```angular2html
tail -n 0 tests/inputs/*
```
如果只有一个文件无输出，如果有多个文件，输出文件标头
拒绝解析非整数的-n或者-c参数


```angular2html
time tail 1M.txt > /dev/null
```
用来做系统测试，time输出后续命令所花费的真实时间，Cpu用户模式实践和CPU内核时间
其中/dev/null是一个特殊设置，重定向到这里可以忽略命令的标准输出到屏幕的显示
系统测试可以适用hyperfine二进制堡检测