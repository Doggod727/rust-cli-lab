# How grep Works
``grep``工具用于从输入中查找匹配给定正则表达式的行。
默认情况下输入来自标准输入，但是也可以通过递归选项指定一个或者多个文件或者目录的名称。查找
目录中所有的文件。默认为查找匹配的行，但也可查找不匹配的行。

```angular2html
grep fox empty.txt
```
grep将第一个位置参数作为正则表达式接受，后续可接入可选的输入文件。
对于空的正则表达式将匹配所有输入行。
```angular2html
grep "" fox.txt
```
其中``""``为空的正则表达式
```
grep Nobody nobody.txt
```
从nobody.txt文件中搜索包含了Nobody的表达式，对应会特殊标红。
```angular2html
grep -i nobody nobody.txt
```
其中`-i`选项说明忽略大小写进行匹配
```angular2html
grep -v Nobody nobody.txt
```
`-v`选项找到不匹配该模式的行

```angular2html
grep -c Nobody nobody.txt
```
`-c`选项用于计数匹配模式的行数

```angular2html
grep -vc Nobody nobody.txt
```
`-v`和`-c`选项可以一起使用

```angular2html
grep The *.txt
```
输入多个文件时，每一个行都会包含文件名称

```angular2html
grep -c The *.txt
```
计数结果也会包括文件名称

`grep` 无法直接搜索目录的内容
`-r|--recursive`选项可以使得我们查找目录中包含匹配文本的所有文件, 此时接受目录。
`-r -i`可以结合使用

# Getting Started
## Defining the Arguments
位置参数的定义很重要！位置参数在clap解析中存在一个Index, 从1开始，clap解析输入参数时，会将每一个输入参数按照index进行分配。
必选的位置参数的index一定要比可选的小，否则会造成歧义
