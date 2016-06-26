#include <mmio.h>
#include <stdint.h>

extern void mmio_write(uint32_t reg, uint32_t data);
 
extern uint32_t mmio_read(uint32_t reg);
 
extern void delay(int32_t count);
