use arch::io::mmio;
use arch::io::gpio;

//TODO: Need to implement Box/kmalloc.
//trait Uart {
//    fn putc(&self, &char);
//    fn puts(&self, &str);
//    fn write(&self,&[char]);
//    fn getc(&self) -> char;
//}
//
//pub struct Uart {
//    pub mmio_base: usize;
//}

//extern crate std;
//use self::std::os::raw::c_char as c_char;

// The offsets for reach register.

// The base address for UART.
//const UART0_BASE_OFFSET: usize = 0x00001000 as usize;
const UART0_BASE_OFFSET: usize = 0x10009000 as usize;
 
// The offsets for reach register for the UART.
const DR     :*mut usize = (UART0_BASE_OFFSET + 0x00usize) as *mut usize ;
const RSRECR :*mut usize = (UART0_BASE_OFFSET + 0x04usize) as *mut usize ;
const FR     :*mut usize = (UART0_BASE_OFFSET + 0x18usize) as *mut usize ;
const ILPR   :*mut usize = (UART0_BASE_OFFSET + 0x20usize) as *mut usize ;
const IBRD   :*mut usize = (UART0_BASE_OFFSET + 0x24usize) as *mut usize ;
const FBRD   :*mut usize = (UART0_BASE_OFFSET + 0x28usize) as *mut usize ;
const LCRH   :*mut usize = (UART0_BASE_OFFSET + 0x2Cusize) as *mut usize ;
const CR     :*mut usize = (UART0_BASE_OFFSET + 0x30usize) as *mut usize ;
const IFLS   :*mut usize = (UART0_BASE_OFFSET + 0x34usize) as *mut usize ;
const IMSC   :*mut usize = (UART0_BASE_OFFSET + 0x38usize) as *mut usize ;
const RIS    :*mut usize = (UART0_BASE_OFFSET + 0x3Cusize) as *mut usize ;
const MIS    :*mut usize = (UART0_BASE_OFFSET + 0x40usize) as *mut usize ;
const ICR    :*mut usize = (UART0_BASE_OFFSET + 0x44usize) as *mut usize ;
const DMACR  :*mut usize = (UART0_BASE_OFFSET + 0x48usize) as *mut usize ;
const ITCR   :*mut usize = (UART0_BASE_OFFSET + 0x80usize) as *mut usize ;
const ITIP   :*mut usize = (UART0_BASE_OFFSET + 0x84usize) as *mut usize ;
const ITOP   :*mut usize = (UART0_BASE_OFFSET + 0x88usize) as *mut usize ;
const TDR    :*mut usize = (UART0_BASE_OFFSET + 0x8Cusize) as *mut usize ;

const DELAY: usize = 150;


// TODO: Define a trait for the UART driver.

pub fn init () {

    unsafe {
    //Disable UART
    mmio::write(CR, 0x0);

    //Disable pull up/down for all GPIO pins & wait.
    mmio::write(gpio::GPPUD, 0x00000000);
    mmio::delay(DELAY);

    // Disable pull up/down for pin 14,15 & delay for 150 cycles.
    mmio::write(gpio::GPPUDCLK0, (1 << 14) | (1 << 15));
    mmio::delay(DELAY);

    // Write 0 to GPPUDCLK0 to make it take effect.
    mmio::write(gpio::GPPUDCLK0, 0x00000000);

    // Clear pending interrupts.
    mmio::write(ICR, 0x7FF);

    // Set integer & fractional part of baud rate.
    // Divider = UART_CLOCK/(16 * Baud)
    // Fraction part register = (Fractional part * 64) + 0.5
    // UART_CLOCK = 3000000; Baud = 115200.

    // Divider = 3000000 / (16 * 115200) = 1.627 = ~1.
    // Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
    mmio::write(IBRD, 1);
    mmio::write(FBRD, 40);

    // Enable FIFO & 8 bit data transmissio (1 stop bit, no parity).
    mmio::write(LCRH, (1 << 4) | (1 << 5) | (1 << 6));

    // Mask all interrupts.
    mmio::write(IMSC, (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) |
               (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));

    // Enable UART0, receive & transfer part of UART.
    mmio::write(CR, (1 << 0) | (1 << 8) | (1 << 9));
    }
    
}


pub fn putc(byte: &char)
{
    unsafe {
        // Wait for UART to become ready to transmit.
            while (mmio::read(FR) & (1usize << 5usize) ) > 0 {}
        mmio::write(DR, *byte as usize);
    }
}

pub fn putb(byte: &u8) {
    unsafe {
        // Wait for UART to become ready to transmit.
        while ( mmio::read(FR) & (1usize << 5usize) ) > 0 { ;}
        mmio::write(DR, *byte as usize);
    }
}

pub fn puts(s: &str)
{
    unsafe {
        for c in s.chars() {
            putc(&c);
        }
    }
}

pub fn write(buffer: &[u8]) {
    unsafe {
        for c in buffer {
            putb(c);
        }
    }
}

pub fn getc() -> char
{

    unsafe {
        // Wait for UART to have recieved something.
        while ( mmio::read(FR) & (1 << 4) ) > 0{ }
        mmio::read(DR) as u8 as char
    }
}
