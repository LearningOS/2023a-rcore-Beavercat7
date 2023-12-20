//! SBI console driver, for text output
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

//用于表示标准输出(stdout)
struct Stdout;

//实现了'Write'trait,允许'stdout'类型对象调用标准输出(stdout)
//write_str方法用于将字符串写入到标准输出,在这个实现中,它遍历输入字符串中的每个字符
//并通过console_putchar函数将字符转换为Unicode码点(uszie类型),然后将其输出到控制台
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

//调用了'stdout'结构体的'write_fmt'方法
//使用'fmt_Arguments'对象,将格式化的字符串输出到标准输出。
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
/*
允许以类似于标准库的 print! 和 println! 宏的方式在主机控制台上进行输出。

这些宏采用类似于标准库宏的格式字符串，并将格式化的字符串传递给 print 函数。
*/
/// Print! to the host console using the format string and arguments.
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

/// Println! to the host console using the format string and arguments.
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}
