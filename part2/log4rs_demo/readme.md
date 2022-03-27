# 日志级别
    error > warn > info > debug > trace

# 配置详解

1、appenders 输出到什么地方

a) kind：指定类型

    console：控制台
    file：普通的日志文件
    rolling_file：可以分割处理的日志文件

b) path：指定文件路径

c) append: bool类型，是否拼接到文件尾部

d) encoder：指定日志的输出格式，默认为kind: pattern

    json：json格式输出
    pattern：模式输出，如{d} [{t}] {l} {M}:{m}{n}
    writer

e) policy：日志分割处理的策略

    compound：复合策略，多个策略规则
    trigger：触发策略 kind: size和limit: 1024，按照文件大小，限制1024字节
    roller：分割策略delete，超过1024字节，处理方式是删除，
            也可以使用fixed_window压缩存储