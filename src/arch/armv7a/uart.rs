#[path="mmio.rs"]
mod mmio;

//extern crate std;
//use self::std::os::raw::c_char as c_char;

// The offsets for reach register.
const GPIO_BASE: u32 = 0x3F200000;

// Controls actuation of pull up/down to ALL GPIO pins.
const GPPUD: u32 = (GPIO_BASE + 0x94);

// Controls actuation of pull up/down for specific GPIO pin.
const GPPUDCLK0: u32 = (GPIO_BASE + 0x98);
/*TODO: Move this gpio defs somewhere else.*/
 
// The base address for UART.
const UART0_BASE_OFFSET: u32 = 0x00001000;
 
// The offsets for reach register for the UART.
const DR     :u32 = 0x00 + GPIO_BASE;
const RSRECR :u32 = 0x04 + GPIO_BASE;
const FR     :u32 = 0x18 + GPIO_BASE;
const ILPR   :u32 = 0x20 + GPIO_BASE;
const IBRD   :u32 = 0x24 + GPIO_BASE;
const FBRD   :u32 = 0x28 + GPIO_BASE;
const LCRH   :u32 = 0x2C + GPIO_BASE;
const CR     :u32 = 0x30 + GPIO_BASE;
const IFLS   :u32 = 0x34 + GPIO_BASE;
const IMSC   :u32 = 0x38 + GPIO_BASE;
const RIS    :u32 = 0x3C + GPIO_BASE;
const MIS    :u32 = 0x40 + GPIO_BASE;
const ICR    :u32 = 0x44 + GPIO_BASE;
const DMACR  :u32 = 0x48 + GPIO_BASE;
const ITCR   :u32 = 0x80 + GPIO_BASE;
const ITIP   :u32 = 0x84 + GPIO_BASE;
const ITOP   :u32 = 0x88 + GPIO_BASE;
const TDR    :u32 = 0x8C + GPIO_BASE;

const DELAY: u32 = 150;


// TODO: Define a trait for the UART driver.

pub fn init () {

    unsafe {
    //Disable UART
    mmio::write(&mut CR, 0x0);

    //Disable pull up/down for all GPIO pins & wait.
    mmio::write(&mut GPPUD, 0x00000000);
    mmio::delay(DELAY);

    // Disable pull up/down for pin 14,15 & delay for 150 cycles.
    mmio::write(&mut GPPUDCLK0, (1 << 14) | (1 << 15));
    mmio::delay(DELAY);

    // Write 0 to GPPUDCLK0 to make it take effect.
    mmio::write(&mut GPPUDCLK0, 0x00000000);

    // Clear pending interrupts.
    mmio::write(&mut ICR, 0x7FF);

    // Set integer & fractional part of baud rate.
    // Divider = UART_CLOCK/(16 * Baud)
    // Fraction part register = (Fractional part * 64) + 0.5
    // UART_CLOCK = 3000000; Baud = 115200.

    // Divider = 3000000 / (16 * 115200) = 1.627 = ~1.
    // Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
    mmio::write(&mut IBRD, 1);
    mmio::write(&mut FBRD, 40);

    // Enable FIFO & 8 bit data transmissio (1 stop bit, no parity).
    mmio::write(&mut LCRH, (1 << 4) | (1 << 5) | (1 << 6));

    // Mask all interrupts.
    mmio::write(&mut IMSC, (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) |
               (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));

    // Enable UART0, receive & transfer part of UART.
    mmio::write(&mut CR, (1 << 0) | (1 << 8) | (1 << 9));
    }
    
}


pub fn putc(byte: char)
{
    unsafe {
    // Wait for UART to become ready to transmit.
    while ( mmio::read(&mut FR) & (1u32 << 5u32) ) > 0 { }
    mmio::write(&mut DR, byte as u32);
    }
}

pub fn puts(s: &str)
{
    unsafe {
    for c in s.chars() {
        putc(c);
    }
    }
}
 
//pub fn write(buffer: &str) {
//    unsafe {
//    for c in buffer.chars() {
//        putc(c);
//    }
//    }
//}
//
pub fn getc() -> char
{

    unsafe {
    // Wait for UART to have recieved something.
    while ( mmio::read(&mut FR) & (1 << 4) ) > 0{ }
    mmio::read(&mut DR) as u8 as char
    }
}
