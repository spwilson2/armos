#![crate_name="kernel"]
#![crate_type="staticlib"]
#![feature(rustc_private, start, lang_items, asm, naked_functions, unique, const_fn, core_intrinsics)]
#![allow(unused_variables, unused_unsafe, unused_imports, unused_features, unused_mut)]
#![no_builtins]
#![no_std]
extern crate rlibc;

use arch::drivers::uart;
use arch::drivers::led;
use arch::sections::SectionMap;

#[macro_use]
mod arch;

#[no_mangle]
pub fn main() {
    uart::init();
    println!("Hello world!\r\n");
    //unsafe{ (*section_map).verify() };
    //led::blink();
    loop{}
    //return 1;
}

#[lang = "eh_personality"] 
pub extern fn eh_personality() {}

#[lang = "panic_fmt"] 
#[no_mangle]
pub extern fn rust_begin_panic(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nKERNEL PANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}

// Fix llbv landing pads.
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1 () -> ! {loop{}}
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () -> ! {loop{}}
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_Resume() -> ! {loop{}}
