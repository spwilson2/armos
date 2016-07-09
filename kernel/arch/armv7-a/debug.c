#include <armv7a.h>
extern inline void dump_registers(struct registers*);
void 
inline print_regs_to_serial(void) {
	struct registers regfile;
	dump_registers(&regfile);
}
