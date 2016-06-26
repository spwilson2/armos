#ifndef _MMIO_H_
#define _MMIO_H_
#include <stdint.h>

//inline void mmio_write(uint32_t reg, uint32_t data);
//inline uint32_t mmio_read(uint32_t reg);
//inline void delay(int32_t count);

inline void mmio_write(uint32_t reg, uint32_t data)
{
	*(volatile uint32_t *)reg = data;
}

inline uint32_t mmio_read(uint32_t reg)
{
	return *(volatile uint32_t *)reg;
}

/* Loop <delay> times in a way that the compiler won't optimize away. */
inline void delay(int32_t count)
{
	asm volatile("__delay_%=: subs %[count], %[count], #1; bne __delay_%=\n"
		 : : [count]"r"(count) : "cc");
}
#endif
