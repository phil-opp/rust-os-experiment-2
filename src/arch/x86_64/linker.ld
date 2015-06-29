
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

    .interrupt_handlers :
    {
        /* ensure that the address (interrupt_handlers-0x200000) fits in 16bit*/
        *(.interrupt_handlers)
    }

    kernel_stack_bottom = .;
    . += kernel_stack_size;
    kernel_stack_top = .;

    .text :
    {
      *(.text)
    }
}
