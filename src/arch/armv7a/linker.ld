ENTRY(_start)

SECTIONS
{
    /* Starts at LOADER_ADDR. */
    . = 0x8000;
    /*. = 0x10000; -kernel start in qemu*/
    __start = .;
    __text_start = .;
    .text :
    {
        KEEP(*(.text.boot))
        *(.text .text.*)
        /*(.text .text.*)*/
    }
    .ARM.text (NOLOAD) : {
        *(.ARM.exidx.text.*)
    }
    . = ALIGN(4096); /* align to page size */
    __text_end = .;
 
    __rodata_start = .;
    .rodata :
    {
        *(.rodata .rodata.*)
        KEEP(*(.sections.descriptor))
    }
    . = ALIGN(4096); /* align to page size */
    __rodata_end = .;
 
    __data_start = .;
    .data :
    {
        *(.data .data.*)
    }
    . = ALIGN(4096); /* align to page size */
    __data_end = .;
 
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    . = ALIGN(4096); /* align to page size */
    __bss_end = .;
    __end = .;

    .multiboot : {
        KEEP(*(.strtab))
        KEEP(*(.symtab))
    }
}
