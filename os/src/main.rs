//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

//! 主模块和入口点
//!
//! 操作系统和应用程序也从这个模块开始运行。内核代码从 `entry.asm` 开始执行，然后调用 [`rust_main()`] 来初始化各种功能 [`clear_bss()`]（有关详细信息，请参阅其源代码）。
//!
//! 然后我们调用 [`println!`] 来显示 `Hello, world!`。

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
/*
在程序加载时，BSS 段用于存储未初始化的全局变量和静态变量，并且这些变量在编译时没有被显式地初始化为特定的值。这些变量会默认被设置为零值（0）。这段代码的作用是将 BSS 段的内存空间全部清零。
使用 extern "C" 语法声明了两个外部函数 sbss 和 ebss。这些函数通常是链接器（Linker）提供的，用于表示 BSS 段的起始地址（sbss）和结束地址（ebss）。
使用 (sbss as usize..ebss as usize) 创建了一个范围（Range），表示从 BSS 段的起始地址到结束地址的范围。
使用 .for_each() 方法遍历该范围内的每一个地址。
在遍历过程中，对每个地址执行 unsafe { (a as *mut u8).write_volatile(0) } 操作。这行代码的作用是将地址对应的内存内容设置为零值（0），这里使用了 write_volatile 方法是因为操作系统可能会对这块内存进行优化或缓存，write_volatile 会告知编译器不要进行任何优化。
*/
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
//标注操作系统中各个内存段的边界、起始地址或特定位置。
    extern "C" {
        fn stext(); // begin addr of text segment 文本段的起始地址
        fn etext(); // end addr of text segment 文本段的结束地址
        fn srodata(); // start addr of Read-Only data segment 只读数据段的起始地址
        fn erodata(); // end addr of Read-Only data ssegment  只读数据段的结束地址
        fn sdata(); // start addr of data segment  数据段的起始地址
        fn edata(); // end addr of data segment  数据段的结束地址
        fn sbss(); // start addr of BSS segment  BSS段的起始地址
        fn ebss(); // end addr of BSS segment  BSS段的结束地址
        fn boot_stack_lower_bound(); // stack lower bound  栈的下界
        fn boot_stack_top(); // stack top 栈的顶部
    }
//清零BSS段
    clear_bss();
//初始化日志记录器,并根据环境变量中设定的日志级别,设置日志记录器的输出级别,以便在程序运行时根据不同的需求输出不同级别的日志信息。
    logging::init();
    println!("[kernel] Hello, world!");
//使用了Rust的日志宏(trace!,debug!,info!,warn!,error!)来输出一些内核各个段的地址范围信息
//每个日志宏用于输出不同段的起始地址和结束地址,格式化成相应的字符串
//这些宏通过传入不同的格式化字符串和变量（函数地址转换为 usize），来记录和输出内核中各个段的内存地址范围信息。这些信息对于调试和了解内核运行时的内存布局非常有用。
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    use crate::board::QEMUExit;
    crate::board::QEMU_EXIT_HANDLE.exit_success(); // CI autotest success
                                             //crate::board::QEMU_EXIT_HANDLE.exit_failure(); // CI autoest failed
}
