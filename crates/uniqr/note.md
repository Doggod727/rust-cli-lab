# Den of Uniquity
## How uniq Works uniq
``uniq``将输入中的相邻行中重复的行去除后，输出到输出
如果没有给定输入文件或者'-'，使用标准输入，如果没有指定输出文件，使用标准输出。
相同相邻输入行的第二份及其后续副本不会写入，如果重复行不相邻，则无法被检测到。

``-c`` 在每一个输出行前加上该行的出现次数
``-d`` 只输出输入中重复出现的行
``-f num`` 忽略每一个输入行的前num个字段，也就是非空字符串组成的字符串
``-s chars`` 忽略每一个输入行的前chars个字符
``-u`` 输出输入中不重复的行
``-i`` 不区分大小写
```angular2html
uniq [-c | -d | -u] [-i] [-f num] [-s chars] [input_file [output_file]]
```
## Getting Started
tempfile crate可以用来创建临时文件
tempfile::NamedTempFile获取一个动态生成的临时文件名， 测试完成后自动删除
### Defining the Arguments

