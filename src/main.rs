#![crate_name="kernel"]
#![crate_type="staticlib"]
//#![feature(lang_items, asm, const_fn, naked_functions)]
//#![feature(lang_items, asm, naked_functions)]
#![feature(lang_items, asm)]
#![no_std]

use arch::drivers::uart;
use arch::drivers::led;

mod arch;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

#[no_mangle]
//#[cold]
#[inline(never)]
pub extern "C" fn rust_main() -> !{
    //unsafe {mmio::mmio_write(&mut 0x00000000usize, 0x00000001usize);}
    uart::init();
    uart::puts("Hello World!" as &str);
    //uart::putc(&'h');
    //led::blink();
    
    loop {}
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1 () -> ! {loop{}}

//#[no_mangle]
//pub extern fn 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
