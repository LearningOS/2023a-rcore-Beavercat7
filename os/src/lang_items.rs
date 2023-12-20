//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;

//当发生panic(即程序遇到无法继续执行的错误时,这个函数将被调用来处理panic的情况)
#[panic_handler]
/// panic handler
/*
use crate::sbi::shutdown;：这行代码引入了一个 shutdown 函数，可能用于关闭系统或执行一些系统级的操作。该函数可能是从 sbi 模块中导入的。

use core::panic::PanicInfo;：这行代码引入了 Rust 标准库中 PanicInfo 类型，它包含有关 panic 的信息，例如 panic 发生的位置、消息等。

#[panic_handler]：这个属性标记了下面的函数 panic 是一个 panic 处理函数。在 Rust 中，可以通过 #[panic_handler] 属性来定义自定义的 panic 处理函数。

fn panic(info: &PanicInfo) -> ! { ... }：这是定义的 panic 处理函数。它接收一个 PanicInfo 类型的参数，并且永远不会正常返回（返回类型为 !，表示从不返回的类型）。

函数首先检查 PanicInfo 中是否包含位置信息（文件、行号等），如果有，就打印出发生 panic 的文件名、行号和消息；如果没有位置信息，则仅打印 panic 消息。

然后调用 shutdown() 函数，可能用于关闭系统或执行一些系统级的操作。
*/
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
