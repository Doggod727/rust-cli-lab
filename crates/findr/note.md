# How find Works find
``find``可以在一条或者多条路径中寻找条目，这些条目通过文件，链接和目录以及匹配可选模式的方式进行筛选
默认路径是当前路径，默认表达式是-print
表达式可以包含: 运算符，选项测试和操作
```angular2html
find [-H] [-L] [-P] [-Olevel] [-D help|tree|search|stat|rates|opt|exec] [path...] [expression]
```
``find``必须至少有一个位置参数用来指定要搜索的路径。 对于每一个路径使用递归搜索所有文件和子目录
``find . -type f``使用 ``-type`` 指定选项f并只查找文件
``find . type l``使用选项l只查找链接, d选项之查找目录
``-name``选项可以通过文件通配符模式定位项目
``-o`` 链接多个`-name`模式实现逻辑或
``-type`` 可以和 ``-name``模式组合
可以选择多个搜索路径
如果给定的搜索路径不存在，find打印错误信息，如果是现在文件的名称，find直接打印当前文件
# Getting Started
## Defining the Arguments
文件通配符中，`.`就是一个字面符常量，'*'表示任意的字符.
在正则表达式中, `.`是表示任意字符， `*`表示前一个字符出现0到任意多次
