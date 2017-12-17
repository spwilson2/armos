ENTRY(_start)

SECTIONS
{
    /* Starts at LOADER_ADDR. */
    . = 0x8000;
    /*_start = .;*/
    /*rust_main = .;*/
    .text._start : {
        KEEP(*(.text._start));
    }
    rust_eh_personality = .;
    .text :
    {
        *(.text.*)
        KEEP(*(.keep));
    }
}
