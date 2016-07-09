pub const GPIO_BASE: usize = 0x3F200000;
pub const GPIO_BASE_PTR: *mut usize = GPIO_BASE as *mut usize;

// Controls actuation of pull up/down to ALL GPIO pins.
pub const GPPUD: *mut usize = (GPIO_BASE + 0x94) as *mut usize;

// Controls actuation of pull up/down for specific GPIO pin.
pub const GPPUDCLK0: *mut usize = (GPIO_BASE + 0x98) as *mut usize;
