#![crate_name="kernel"]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(asm)]
#![no_std]

use arch::mmio;
use arch::uart;

mod arch;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

#[no_mangle]
pub extern "C" fn rust_main() {
    //unsafe {mmio::mmio_write(&mut 0x00000000usize, 0x00000001usize);}
    uart::init();
    uart::puts("Hello World!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
