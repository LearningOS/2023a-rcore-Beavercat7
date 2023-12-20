//! SBI call wrappers
// SBI 调用包装器
use core::arch::asm;
//该常量表示 SBI 调用中用于向控制台输出字符的标识符
const SBI_CONSOLE_PUTCHAR: usize = 1;

//这个函数可用于发起不同类型的 SBI 调用，传入参数以及 SBI 调用编号 (which)。
/// general sbi call
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    //1:将常数0装载到寄存器'x16'中
    //2:ecall指令用于触发特权级别切换,将控制器转移到操作系统,以便进行系统调用
    //3:使用Rust内联汇编的输入输出操作约束,它指定了寄存器的使用情况
    //3:表示将参数'arg0'的值传递给寄存器'x10',并且在SBI调用完成后,将'x10'的值传回给'ret'变量
    //4:将arg1的值传递给寄存器'x11'
    //5:将arg2的值传递给寄存器'x12'
    //6:将参数'which'的值传递给寄存器'x17'
    unsafe {
        asm!(
            "li x16, 0",
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}
// SBI 调用来向控制台输出字符。它调用了 sbi_call 函数，传入 SBI_CONSOLE_PUTCHAR 作为 SBI 调用的标识符，并传入字符的 Unicode 值 c，实现向控制台输出。
/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

use crate::board::QEMUExit;
//这个函数用于关闭内核。它调用了 QEMU_EXIT_HANDLE 对象的 exit_failure() 方法，可能是用于通知 QEMU 虚拟机内核的退出状态为失败状态，以触发内核的关闭操作。这个函数返回 ! 类型，表示它永远不会正常返回，因为内核已经关闭。
/// use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    crate::board::QEMU_EXIT_HANDLE.exit_failure();
}
