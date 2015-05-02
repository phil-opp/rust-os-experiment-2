
ENTRY(start)

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
}