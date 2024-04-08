/*
 * @Author: LinkyPi trouble.linky@gmail.com
 * @Date: 2024-04-06 16:35:30
 * @LastEditors: LinkyPi trouble.linky@gmail.com
 * @LastEditTime: 2024-04-06 17:54:21
 * @FilePath: /myos/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

// 不链接 Rust 标准库
 #![no_std]

// 禁用所有 Rust 层级的入口点
#![no_main]

use core::panic::PanicInfo;

/// 这个函数将在 panic 时被调用
#[cfg(not(test))]
#[panic_handler]
fn panic(_info : &PanicInfo) -> !{
    loop{}
}

// 使用#[no_mangle] 这个标注属性后，编译器就不会修改它们的名字了。mangling 是一个特殊的编译阶段，
// 在这个阶段，编译器会修改函数名称来包含更多用于后续编译步骤的信息，但通常也会使得函数名称难以阅读
// 因为链接器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
// 默认命名为 `_start`
#[no_mangle]
pub extern "C" fn _start()->!{

    static HELLO: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // 淡青色
        }
    }
    loop{}
}