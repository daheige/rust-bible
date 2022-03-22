# env_log
    The log levels that may be specified correspond to the log::Level 
    enum from the log crate. They are:
    error
    warn
    info
    debug
    trace
    日志level 优先级  error > warn > info > debug > trace

    By default, env_logger logs to stderr. 
    If you want to log to stdout instead, you can use the Builder 
    to change the log target:
