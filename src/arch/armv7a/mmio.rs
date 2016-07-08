pub unsafe fn mmio_write(address: *mut usize, value: usize) {
    *address = value;
}
