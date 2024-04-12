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


// 自定义测试框架
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;

mod vga_buffer;
use core::panic::PanicInfo;

// 这个函数将在 panic 时被调用
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    myos::hlt_loop();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myos::test_panic_handler(info)
}

use bootloader::{BootInfo, entry_point};

// entry_point 增加了函数签名检查， 而原始的 extern "C" fn _start 并未做该处理
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("Hello World{}", "!");

    myos::init();

    use myos::memory;
    use myos::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, structures::paging::Translate, VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // new: initialize a mapper
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
     // 映射未使用的页
     let page = Page::containing_address(VirtAddr::new(0));
     memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
 
     // 通过新的映射将字符串 `New!`  写到屏幕上。
     let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
     // 白色背景上的字符串 “New!” 
     unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     // new: use the `mapper.translate_addr` method
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }


    // 打印三级及四级页表
    // use myos::memory::active_level_4_table;
    // use x86_64::VirtAddr;
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         // get the physical address from the entry and convert it
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         // print non-empty entries of the level 3 table
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    #[cfg(test)]
    test_main();

    myos::hlt_loop();
}

// 使用#[no_mangle] 这个标注属性后，编译器就不会修改它们的名字了。mangling 是一个特殊的编译阶段，
// 在这个阶段，编译器会修改函数名称来包含更多用于后续编译步骤的信息，但通常也会使得函数名称难以阅读
// 因为链接器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
// 默认命名为 `_start`, 但_start是从引导程序中调用的，所以没有对我们的函数签名进行检查
// 这意味着我们可以让它接受任意参数而不出现任何编译错误，但在运行时它会失败或导致未定义行为。
// 为了确保入口点函数总是具有引导程序所期望的正确签名，bootloader 板块提供了一个 entry_point宏
// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo)->!{

//     // 宏位于根命名空间下
//     println!("Hello World{}", "!");

//     myos::init();

//     use x86_64::registers::control::Cr3;

//     let (level_4_page_table, _) = Cr3::read();
//     println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

//     // // Note: The actual address might be different for you. Use the address that
//     // // your page fault handler reports.
//     // let ptr = 0x2031b2 as *mut u8;

//     // // read from a code page
//     // unsafe { let x = *ptr; }
//     // println!("read worked");

//     // // write to a code page
//     // unsafe { *ptr = 42; }
//     // println!("write worked");

//     // invoke a breakpoint exception
//     // x86_64::instructions::interrupts::int3();

//     // trigger a page fault
//     // unsafe {
//     //     *(0xdeadbeef as *mut u8) = 42;
//     // };

//     #[cfg(test)]
//     test_main();

//     myos::hlt_loop();
// }

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}