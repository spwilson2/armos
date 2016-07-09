.section .text
.global dump_registers
dump_registers:
mov r3, #1
stm r0, {r0-r15}
bx   lr
