pub unsafe fn mmio_write(address: *mut usize, value: usize) {
    *address = value;
}

pub unsafe fn mmio_read(address: *mut usize) -> usize {
    *address
}
