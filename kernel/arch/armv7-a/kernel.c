#if !defined(__cplusplus)
#include <stdbool.h>
#endif
#include <stddef.h>
#include <stdint.h>
#include <uart.h>
 
#if defined(__cplusplus)
extern "C" /* Use C linkage for kernel_main. */
#endif
void kernel_main(uint32_t r0, uint32_t r1, uint32_t atags)
{
	(void) r0;
	(void) r1;
	(void) atags;
 

	gUart->init(gUart);
	gUart->puts(gUart, "Hello, kernel World!\r\n");
 
	while ( true )
		gUart->putc(gUart, gUart->getc(gUart));
}
