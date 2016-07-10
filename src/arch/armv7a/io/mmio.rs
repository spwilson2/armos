use core;


pub unsafe fn write(address: *mut usize, value: usize) {
    core::ptr::write_volatile(address, value)
}

pub unsafe fn read(address: *mut usize) -> usize {
    core::ptr::read_volatile(address)
}

pub fn delay(time: usize) {
    unsafe {
        for mut i in 0..time {
            //core::intrinsics::atomic_and(&mut i,i);
            asm!("and r1,r1"::::"volatile");
        }
    }
}
