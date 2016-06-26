#ifndef _UART_H_
#define _UART_H_
/*TODO: Move this gpio defs somewhere else.*/
// The GPIO registers base address.
//GPIO_BASE = 0x20200000, For rpi single core
#define GPIO_BASE   0x3F200000

// The offsets for reach register.

// Controls actuation of pull up/down to ALL GPIO pins.
#define GPPUD       (GPIO_BASE + 0x94)

// Controls actuation of pull up/down for specific GPIO pin.
#define GPPUDCLK0   (GPIO_BASE + 0x98)
/*TODO: Move this gpio defs somewhere else.*/
 
// The base address for UART.
#define UART0_BASE_OFFSET   0x00001000
 
// The offsets for reach register for the UART.
#define UART0_DR_OFFSET     0x00
#define UART0_RSRECR_OFFSET 0x04
#define UART0_FR_OFFSET     0x18
#define UART0_ILPR_OFFSET   0x20
#define UART0_IBRD_OFFSET   0x24
#define UART0_FBRD_OFFSET   0x28
#define UART0_LCRH_OFFSET   0x2C
#define UART0_CR_OFFSET     0x30
#define UART0_IFLS_OFFSET   0x34
#define UART0_IMSC_OFFSET   0x38
#define UART0_RIS_OFFSET    0x3C
#define UART0_MIS_OFFSET    0x40
#define UART0_ICR_OFFSET    0x44
#define UART0_DMACR_OFFSET  0x48
#define UART0_ITCR_OFFSET   0x80
#define UART0_ITIP_OFFSET   0x84
#define UART0_ITOP_OFFSET   0x88
#define UART0_TDR_OFFSET    0x8C

//To access each register just use a macro.
#define UART0_REG(reg, device_ptr)  UART0_##reg##_OFFSET + device_ptr->uart_base

typedef struct _uart_device {
	uint uart_base;
	void (*init)    (struct _uart_device *);
	void (*putc)    (struct _uart_device *, uchar);
	void (*puts)    (struct _uart_device *, const char *);
	uchar (*getc)   (struct _uart_device *);
} uart_device;

void uart_driver_init(void);

uart_device * gUart;

#endif
