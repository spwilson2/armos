#![crate_name="kernel"]
#![crate_type="staticlib"]
//#![feature(lang_items, asm, const_fn, naked_functions)]
#![feature(lang_items, asm, naked_functions)]
//#![feature(lang_items, asm)]
#![no_std]

use arch::drivers::uart;
use arch::drivers::led;

mod arch;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

#[no_mangle]
#[inline(never)]
pub extern "C" fn rust_main() -> !{
    uart::init();
    uart::puts("Hello Kernel World!" as &str);
    led::blink();
    loop{}
}

// Fix llbv landing pads.
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1 () -> ! {loop{}}
