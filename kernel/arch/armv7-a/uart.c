#include <uart.h>
#include <stddef.h>
#include <string.h>
#include <stdint.h>
#include <mmio.h>

static void uart_init(uart_device * this);
static void uart_putc(uart_device * this, uchar byte);
static void uart_puts(uart_device * this, const char * str);
static void uart_write(uart_device * this, const uchar * buffer, size_t size);
static uchar uart_getc(uart_device * this);

/* A global uart device, must call init on it, however no need to call
 * uart_driver_init to create a new device.*/
static uart_device gUartDevice = {
	.uart_base = UART0_BASE_OFFSET + GPIO_BASE,
	.init = &uart_init,
	.putc = &uart_putc,
	.puts = &uart_puts,
	.getc = &uart_getc
};
uart_device * gUart = &gUartDevice;

/*TODO*/
void
uart_driver_init(void) {
}

static void
uart_init(uart_device * this)
{
	// Disable UART0.
	mmio_write(UART0_REG(CR, this), 0x00000000);
	// Setup the GPIO pin 14 && 15.
 
	// Disable pull up/down for all GPIO pins & delay for 150 cycles.
	mmio_write(GPPUD, 0x00000000);
	delay(150);
 
	// Disable pull up/down for pin 14,15 & delay for 150 cycles.
	mmio_write(GPPUDCLK0, (1 << 14) | (1 << 15));
	delay(150);
 
	// Write 0 to GPPUDCLK0 to make it take effect.
	mmio_write(GPPUDCLK0, 0x00000000);
 
	// Clear pending interrupts.
	mmio_write(UART0_REG(ICR, this), 0x7FF);
 
	// Set integer & fractional part of baud rate.
	// Divider = UART_CLOCK/(16 * Baud)
	// Fraction part register = (Fractional part * 64) + 0.5
	// UART_CLOCK = 3000000; Baud = 115200.
 
	// Divider = 3000000 / (16 * 115200) = 1.627 = ~1.
	// Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
	mmio_write(UART0_REG(IBRD, this), 1);
	mmio_write(UART0_REG(FBRD, this), 40);
 
	// Enable FIFO & 8 bit data transmissio (1 stop bit, no parity).
	mmio_write(UART0_REG(LCRH, this), (1 << 4) | (1 << 5) | (1 << 6));
 
	// Mask all interrupts.
	mmio_write(UART0_REG(IMSC, this), (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) |
	                       (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));
 
	// Enable UART0, receive & transfer part of UART.
	mmio_write(UART0_REG(CR, this), (1 << 0) | (1 << 8) | (1 << 9));
}

static void
uart_putc(uart_device * this, uchar byte)
{
	// Wait for UART to become ready to transmit.
	while ( mmio_read(UART0_REG(FR, this)) & (1 << 5) ) { }
	mmio_write(UART0_REG(DR, this), byte);
}

static void
uart_puts(uart_device * this, const char * str)
{
	uart_write(this, (const uchar*) str, strlen(str));
}
 
static void
uart_write(uart_device * this, const uchar * buffer, size_t size)
{
	for ( size_t i = 0; i < size; i++ )
		uart_putc(this, buffer[i]);
}

static uchar
uart_getc(uart_device * this)
{
    // Wait for UART to have recieved something.
    while ( mmio_read(UART0_REG(FR, this)) & (1 << 4) ) { }
    return mmio_read(UART0_REG(DR, this));
}
