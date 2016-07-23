// A non-spec version of the multiboot header that way I can print frames..
.section ".sections.descriptor"
.globl __sections_header

// NOTE: All sections must be set up before the header, since it uses SECTIONS 
// which is incremented every time a SECTION_ENTRY is added.
.set SECTIONS, 0

// Create a struct of info for each section
.macro section_entry name
.set SECTIONS, SECTIONS + 1
.int __\name\()_start
.int __\name\()_end
.int __\name\()_string //Pointer to null terminated ascii string.
.int __\name\()_strlen //Length of the string (Needed by rust.)
.endm

// Create a null terminated ascii string for each section
.macro section_string name
__\name\()_string:
.ascii "\name\0"
.set __\name\()_strlen, .-__\name\()_string
.endm

// Array of section entry structs.
.align 4
sections_list:
	 SECTION_ENTRY text
	 SECTION_ENTRY rodata
	 SECTION_ENTRY data
	 SECTION_ENTRY bss

// Strings for each section entry.
SECTION_STRING text
SECTION_STRING rodata
SECTION_STRING data
SECTION_STRING bss

// Keep a magic value so we know the descriptor table header is loaded correctly.
.set SECTION_MAP_MAGIC, 0xD1CE0000

.balign 4
__sections_header:
	.int SECTION_MAP_MAGIC
	.int SECTIONS
	.int sections_list


// To keep this in the first portion of the binary.
.section ".text.boot"
 
// Make _start global.
.globl _start
 
// Entry point for the kernel.
// r15 -> should begin execution at 0x8000.
// r0 -> 0x00000000
// r1 -> 0x00000C42
// r2 -> 0x00000100 - start of ATAGS
// preserve these registers as argument for kernel_main
_start:
	// Setup the stack.
	mov sp, #0x8000
 
	// Clear out bss.
	ldr r4, =__bss_start
	ldr r9, =__bss_end
	mov r5, #0
	mov r6, #0
	mov r7, #0
	mov r8, #0
	b       2f
 
1:
	// store multiple at r4.
	stmia r4!, {r5-r8}
 
	// If we are still below bss_end, loop.
2:
	cmp r4, r9
	blo 1b
 
	// Store the end of the kernel as an argument to main.
	ldr r0, =__sections_header

	// Call kernel_main
	ldr r3, =rust_main
	blx r3
 
	// halt
halt:
	wfe
	b halt
