#ifndef _ARM7A_H_
#define _ARM7A_H_
//#include <dump.h>
struct registers {
	unsigned int r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,r13,r14,r15;
};
inline void dump_registers(regfile);

#endif
