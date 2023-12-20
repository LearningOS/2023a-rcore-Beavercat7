//! Global logger

use log::{self, Level, LevelFilter, Log, Metadata, Record};

/// a simple logger
struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    fn flush(&self) {}
}

/// initiate logger
/*
这段代码用于初始化日志记录器，并根据环境变量中设定的日志级别，设置日志记录器的输出级别，以便在程序运行时根据不同的需求输出不同级别的日志信息。
创建一个静态变量 LOGGER，其类型为 SimpleLogger。这个静态变量将会持有一个 SimpleLogger 实例。

调用 log::set_logger(&LOGGER).unwrap() 将创建的 LOGGER 设置为全局默认的日志记录器。这样，在程序的其他地方通过 log 库记录日志时，将使用该日志记录器来输出日志信息。

调用 log::set_max_level() 设置日志记录器的最大日志级别。它根据环境变量 "LOG" 的值，匹配不同的日志级别设置。具体来说，根据环境变量的不同取值，可以设置日志记录器的级别为 Error、Warn、Info、Debug、Trace 或 Off。这些级别按照严重程度依次递增，Off 表示禁用日志记录，Error 表示只记录错误信息，Trace 表示记录最详细的追踪信息。
*/
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}
