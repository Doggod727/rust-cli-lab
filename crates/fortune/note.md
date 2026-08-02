# How fortune Works
`fortune`程序随机从文本文件数据库中打印一句箴言冷知识。
``
fortune [-acefilosuw] [-n length] [-m pattern] [[n%] file/dir/all]
``
如果运行fortune时没有任何参数，随机打印一个箴言。

``
fortune tests/inputs/ascii-art
``
运行fortune时可以指定一个文件，来随机选择一个箴言。
注意该文件需要简历索引文件。才能进行随机选择。

``
fortune tests/inputs
``
提供一个`tests/inputs`目录，让fortune从其中的任意文件中选取记录

``
fortune tests/inputs/jokes blargh
``
如果提供的路径不存在，fortune会立即报错停止.

``
fortune hammer
``
如果提供的路径不可读，可能会输出不存在，即使该文件真的存在。

``
fortune -m 'Yogi Berra' tests/inputs
``
`-m` 可以搜索所有的匹配指定字符串的文本记录。并且输出时
STDERR: 文件名标题
STDOUT: 记录内容

默认搜索区分大小写，`-i`可以用来执行不区分大小写的匹配。

