pub unsafe fn write(address: *mut usize, value: usize) {
    *address = value;
}

pub unsafe fn read(address: *mut usize) -> usize {
    *address
}

pub unsafe fn delay(time: usize) {
    for i in 0..time {
        ;
    }
}
