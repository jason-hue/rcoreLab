#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod lang_items;
mod sbi;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
#[no_mangle]
pub fn rust_main(){
    clear_bss();
    println!("hello,world!");
    panic!("shutdown machine!");
    loop{}
}
fn clear_bss(){
    extern "C"{
        fn ebss();
        fn sbss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    });
}
