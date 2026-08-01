# How comm Works
``comm`` 工具读取两个文件，输入两个文件共同出现的文本行以及各自独有的文本行，也就是集合操作。
共同出现的文本行是交集，独有行是差集。
在GNU版本中，逐行比较已排序的文件1和文件2.
如果存在一个文件为-时，从标准输入读取，但是不能同时都为-。
无选项输入时，生成三列输出。
列1 => 文件1独有的行。
列2 => 文件2独有的行。
列3 => 文件1，2共有的行。

选项
`-1` 禁止显示第一列
`-2` 禁止显示第二列
`-3` 禁止显示第三列
`--check-order` 判断输入是否正确排序了
`--nocheck-order` 不检查是否正确排序了

```angular2html
comm -12 <(sort cities1.txt) <(sort cities2.txt)
```
只输出cities1.txt 和 cities2.txt的共同文件行，输入前要排序

```angular2html
comm -23 <(sort cities1.txt) <(sort cities2.txt)
```
只输出cities1.txt独有的文本行

```angular2html
sort cities2.txt | comm -12 <(sort cities1.txt) -
```
第一个或者第二个文件可以时标准输入，用(-)表示文件名称

