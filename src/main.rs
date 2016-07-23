#![crate_name="kernel"]
//#![crate_type="bin"]
#![crate_type="staticlib"]
//#![crate_type="rlib"]
//#![feature(lang_items, asm, const_fn, naked_functions)]
#![feature(lang_items, asm, naked_functions, unique, const_fn, core_intrinsics)]
//#![feature(lang_items, asm)]
#![no_std]
extern crate rlibc;

use arch::drivers::uart;
use arch::drivers::led;

#[macro_use]
mod arch;
//mod rlibc;

// Allow this to be called from asm boot and not be optimized out.
#[link(name="rust_main", kind = "static")]
#[no_mangle]
#[cold]
#[inline(never)]
pub extern "cdecl" fn rust_main(end: usize) -> !{
    //uart::init();
    //let s = "Hello Kernel world!\n";
    //uart::puts(&s);
    //let s = b"Hello again!";
    //uart::write(s);
    //let i = 0xffffffffusize;
    //let j= i + 1;
    //print!("Hello again again.\n");
    print!("End: {:X}", end);
    led::blink();
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"] 
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
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
