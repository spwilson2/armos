.section .keep
interrupt_vector_table:
b . @ Reset
b . 
b . @ SWI instruction
b . 
b .
b .
b .
b .

.section .text._start
.globl _start
_start:
ldr sp, = 0x10000 @ Set up the stack
bl main @ Jump to the main function

loop:
b loop
