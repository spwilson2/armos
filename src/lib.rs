#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern fn rust_main() {}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ()
{
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
