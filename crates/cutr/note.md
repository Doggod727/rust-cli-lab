# How cut Works cut
该cut程序没有短选项并且长选项以单破折号开头的程序之一

cut程序的作用 => 从文件或者标准输入中裁剪文本，被选中的文本可以是字节或者字符的某一个范围
也可以是由逗号或者制表符等分隔符划分的字段范围
直白说cut是将选定行的选定部分打印
``-b, --bytes=LIST`` 从每一个文件中选取行的选定范围的字节
``-c, --chars=LIST`` 选定范围内的字符
``-d, --delimiter=DELIM`` 使用DELIM作为分隔符
``-f, --fields=LIST`` 选定字段，同时打印任何不包含分隔符的行

```
    cut -c 1-20 books.txt
```
表示从books.txt文件中的每一行选取前1到20个字符进行打印
```angular2html
cut -c 26-55,1-20 books.txt
```
实际输出时还是按照原始顺序升序排列，不会先输出26-55范围的字符
```angular2html
cut -c 1 books.txt
```
可以使用-c 1选择第一个字符

默认情况下cut假定制表符时字段分隔符
```angular2html
cut -f 2,3 books.tsv
```
表示选择以制表符作为分割的第2，3两个字段
```angular2html
cut -d , -f 2,1 books.csv
```
表示将','视为分隔符，选择第1，2两个字段打印
```angular2html
cut -f foo,bar books.tsv
```
非法，LIST必须接受整数值
如果没有输入文件或者输入文件名时'-'，默认从标准输入读取输入