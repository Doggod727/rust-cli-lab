# Word to Your Mother
## How wc Works wc
``wc``命令用于展示输入的每一个文件或者标准输入的单词数目，行数，字节数。
行：用换行符结尾的字符串；
词：字符串中用空白字符分割的每一个部分；
空白字符集使用``iswspace()``可以返回true
如果有多个文件输入，最后的输出会有一个累计的计数结构输出

``-c`` 将每一个文件的字节数目输出到标准输出。
``-l`` 将每一个文件的行数输出.
``-m`` 将每一个文件的字符数输出。
``-w`` 将每一个文件的词数输出。
指定选项时就只输出选定的选项，输出顺序为 行数单词书字节数文件名
默认行为同时指定-c -l -w
如果没有指定文件名只是用标准输入，不显示文件名
## Getting Started
Iterator::any 至少有一个元素满足闭包，返回true;
Iterator::filter 找到所有谓词为true的元素，返回一个新的迭代器
Iterator::map 对每一个元素应用闭包，返回包含已转换元素的std::iter::Map
Iterator::find 找到迭代器中满足谓词的第一个元素形式为Some(value)
Iterator::position 返回满足谓词的第一个索引
Iterator::cmp Iterator::min_by Iterator::max_by
### Iterating the Files
