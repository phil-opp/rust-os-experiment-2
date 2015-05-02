
ENTRY(start)

kernel_stack_size = 1M;

SECTIONS {

    /* somehow needed for grub to recognize the multiboot header */
    . = 2M + SIZEOF_HEADERS;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        *(.multiboot)
    }
    .tss :
    {
        /* ensure that the address (tss-0x200000) fits in 16bit */
        *(.tss)
    }

    .text :
    {
      *(.text)
      *(.text.*)
    }

    .rodata :
    {
      *(.rodata)
      *(.rodata.*)
    }

    .data :
    {
      *(.data)
    }

    .bss :
    {
     *(.bss)
     *(COMMON)
    }

    other :
    {
      *(*)
    }

    kernel_stack_bottom = .;
    . += kernel_stack_size;
    kernel_stack_top = .;

    kernel_end = .;

}