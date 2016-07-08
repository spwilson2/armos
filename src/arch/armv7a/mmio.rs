pub unsafe fn write(address: *mut u32, value: u32) {
    *address = value;
}

pub unsafe fn read(address: *mut u32) -> u32 {
    *address
}

pub unsafe fn delay(time: u32) {
    for i in 0..time {
        ;
    }
}
