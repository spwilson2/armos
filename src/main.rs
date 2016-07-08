#![feature(lang_items)]
#![feature(asm)]
#![no_std]

#[path="arch/armv7a/mmio.rs"]
pub mod mmio;

#[path="arch/armv7a/uart.rs"]
pub mod uart;

#[no_mangle]
pub extern fn rust_main() {
    //unsafe {mmio::mmio_write(&mut 0x00000000usize, 0x00000001usize);}
    uart::init();
    uart::puts("Hello World!");
}

#[no_mangle]
//extern fn __aeabi_unwind_cpp_pr0() -> ()
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ()
{
    loop {}
}

//#[no_mangle]
//extern fn rust_begin_unwind() -> ! {loop{}}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
